use crate::download;
use std::path::{Path, PathBuf};
use std::io::{Read, Write, BufWriter, BufRead};

use curl::easy::{Easy2, Handler, WriteError};

// https://docs.rs/curl/0.4.38/curl/easy/trait.Handler.html
struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

pub const DEFAULT_FIELDS: &str = "fields=id,title,contentHandler,links"; // Looks like all contentHandlers have these fields (not attachments, though).
const SYNC_CHANNEL_BUFFER_SIZE: usize = 1000000;

#[derive(Debug)]
pub struct BBSession {
    pub domain: String,
    pub cookie_jar_path: PathBuf,
}

impl BBSession {
    pub fn initiate_bb_session(domain: &str, cookie_jar_path: &Path) -> Result<BBSession, Box<dyn std::error::Error>> {
        // https://tech.saigonist.com/b/code/how-login-any-website-using-curl-command-line-or-shell-script.html
        
        eprintln!("Not checking for existing sessions.");

        let mut easy = Easy2::new(Collector(Vec::new()));
        easy.cookie_jar(cookie_jar_path)?;
        easy.cookie_file(cookie_jar_path)?;
        easy.follow_location(true)?; //Viktig fordi BB redirecter (302)
        easy.fail_on_error(true)?; //Viktig for å faile på 401
        
        easy.url("https://ntnu.blackboard.com/ultra")?;
        easy.perform()?; //Husk at denne er synchronous derfor er det trygt å aksessere mutexen under.

        let content = String::from_utf8(easy.get_ref().0.clone()).expect("Error converting content to String");
        let document = scraper::Html::parse_document(&content);
        let nonce_selector = scraper::Selector::parse(r#"input[name="blackboard.platform.security.NonceUtil.nonce"]"#).expect("Error parsing selector");

        let nonce = document.select(&nonce_selector).next().expect("No elements matching selector").value().attr("value").expect("Error getting attribute");

        let stdin = std::io::stdin();
        let mut lines = stdin.lock().lines().flatten();
        println!("Please enter user_id:");
        let user_id = lines.next().unwrap();
        println!("Please enter password:");
        let password = lines.next().unwrap();
        
        let login_form_data = format!("user_id={}&password={}&blackboard.platform.security.NonceUtil.nonce={}", user_id, password, nonce);
        
        easy.url("https://ntnu.blackboard.com/webapps/login/")?;
        easy.post(true)?; // Kanskje unødvendig
        easy.post_fields_copy(login_form_data.as_bytes())?; 
        easy.perform()?; //Husk at denne er synchronous!

        Ok(BBSession {
            domain: domain.to_string(),
            cookie_jar_path: cookie_jar_path.to_path_buf(),
        })
    }

    pub fn download_file(&self, url: &str, out_path: &Path, overwrite: bool) -> Result<f64, Box<dyn std::error::Error>> {
        download::download_file(&url, &out_path, Some(&self.cookie_jar_path), overwrite)
    }

    pub fn download_and_unzip(&self, url: &str, out_path: &Path, overwrite: bool) -> Result<f64, Box<dyn std::error::Error>> {
        download::download_and_unzip(&url, &out_path, Some(&self.cookie_jar_path), overwrite)
    }

    fn download_course_contents_json(&self, course_id: &str, query_parameters: Option<&[&str]>, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        let mut url = format!("https://{}/learn/api/public/v1/courses/{}/contents",
            self.domain,
            course_id);
        
        if let Some(query_parameters) = query_parameters {
            url.extend(format!("?{}", query_parameters.join("&")).chars());
        }

        self.download_file(&url, out_path, true)
    }

    pub fn download_course_root_contents_json(&self, course_id: &str, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        self.download_course_contents_json(course_id, Some(&[DEFAULT_FIELDS]), out_path)
    }

    pub fn download_content_children_json(&self, course_id: &str, content_id: &str, query_parameters: Option<&[&str]>, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        let mut url = format!("https://{}/learn/api/public/v1/courses/{}/contents/{}/children",
            self.domain,
            course_id,
            content_id);
    
        if let Some(query_parameters) = query_parameters {
            url.extend(format!("?{}", query_parameters.join("&")).chars());
        }

        self.download_file(&url, out_path, true)
    }

    pub fn download_course_files_json(&self, course_id: &str, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        self.download_course_contents_json(course_id, Some(&["contentHandler=resource/x-bb-file","recursive=true", DEFAULT_FIELDS]), out_path)
    }

    pub fn download_course_documents_json(&self, course_id: &str, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        self.download_course_contents_json(course_id, Some(&["contentHandler=resource/x-bb-document","recursive=true", DEFAULT_FIELDS]), out_path)
    }

    pub fn download_course_assignments_json(&self, course_id: &str, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        self.download_course_contents_json(course_id, Some(&["contentHandler=resource/x-bb-assignment","recursive=true", DEFAULT_FIELDS]), out_path)
    }
    
    // Add query_parameters argument!
    pub fn download_course_announcements_json(&self, course_id: &str, limit: usize, offset: usize, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        // let fields = "id,title,contentHandler"; Alle egentlig interessante
        
        let url = format!("https://{}/learn/api/public/v1/courses/{}/announcements?limit={}&offset={}",
            self.domain,
            course_id,
            limit,
            offset);
                
        self.download_file(&url, &out_path, true)
    }
    
    pub fn download_content_attachments_json(&self, course_id: &str, content_id: &str, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        
        let url = format!("https://{}/learn/api/public/v1/courses/{}/contents/{}/attachments",
            self.domain,
            course_id,
            content_id);
    
        self.download_file(&url, &out_path, true)
    }

    pub fn download_content_attachment(&self, course_id: &str, content_id: &str, attachment_id: &str, out_path: &Path, unzip: bool, overwrite: bool) -> Result<f64, Box<dyn std::error::Error>> {
        eprintln!("Downloading attachment {:?}", out_path);

        let url = format!("https://{}/learn/api/public/v1/courses/{}/contents/{}/attachments/{}/download",
            self.domain,
            course_id,
            content_id,
            attachment_id);
        
        if unzip {
            self.download_and_unzip(&url, &out_path, overwrite)
        } else {
            self.download_file(&url, &out_path, overwrite)
        }
    }
    
    // pub fn download_course_assessment_questions_json(...)
}