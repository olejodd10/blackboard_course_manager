mod download;
pub mod filename_utils;
pub mod input_utils;

use filename_utils::cookie_filename;
use input_utils::stdin_trimmed_line;

use std::path::{Path, PathBuf};
use std::io::{Read, Write, BufWriter, BufRead};

use curl::easy::{Easy2, Easy, Handler, WriteError};

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
    pub cookie_jar_dir: PathBuf,
    pub cookie_jar_path: PathBuf,
}

impl BBSession {
    pub fn new(domain: &str, cookie_jar_dir: &Path) -> Result<BBSession, Box<dyn std::error::Error>> {
        std::fs::create_dir_all(cookie_jar_dir).expect("Error creating BBSession cookie_jar_dir");
        let cookie_jar_path = cookie_jar_dir.join(cookie_filename(domain));
        let bb_session = BBSession {
            domain: domain.to_string(),
            cookie_jar_dir: cookie_jar_dir.to_path_buf(),
            cookie_jar_path,
        };
        bb_session.connect()?;
        Ok(bb_session)
    }

    pub fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.test_connection() {
            Ok(())
        } else {
            eprintln!("Initiating new BlackBoard connection.");
    
            std::fs::create_dir_all(&self.cookie_jar_dir)?;
            let cookie_jar_path = self.cookie_jar_dir.join(cookie_filename(&self.domain));
            if cookie_jar_path.exists() {
                std::fs::remove_file(&cookie_jar_path)?; // ~
            }
    
            //Block to force cookie file write before connection test
            {
                let mut easy = Easy2::new(Collector(Vec::new()));
                easy.cookie_jar(&cookie_jar_path)?;
                easy.cookie_file(&cookie_jar_path)?;
                easy.follow_location(true)?; //Viktig fordi BB redirecter (302)
                easy.fail_on_error(true)?; //Viktig for å faile på 401
                
                
                easy.url(&format!("https://{}/ultra", self.domain))?;
                easy.perform()?; //Husk at denne er synchronous derfor er det trygt å aksessere mutexen under.
                
                let content = String::from_utf8(easy.get_ref().0.clone()).expect("Error converting content to String");
                let document = scraper::Html::parse_document(&content);
                let nonce_selector = scraper::Selector::parse(r#"input[name="blackboard.platform.security.NonceUtil.nonce"]"#).expect("Error parsing selector");
        
                let nonce = document.select(&nonce_selector).next().expect("No elements matching selector").value().attr("value").expect("Error getting attribute");
        
                let user_id = if let Ok(user_id) = std::env::var("BBCM_USER_ID") {
                    user_id
                } else {
                    println!("Please enter user_id:");
                    stdin_trimmed_line()
                };
                
                println!("Please enter password:");
                let password = stdin_trimmed_line();
                
                let login_form_data = format!("user_id={}&password={}&blackboard.platform.security.NonceUtil.nonce={}", user_id, password, nonce);
                
                easy.url(&format!("https://{}/webapps/login/", self.domain))?;
                easy.post(true)?; // Kanskje unødvendig
                easy.post_fields_copy(login_form_data.as_bytes())?; 
                easy.perform()?; //Husk at denne er synchronous!
            }
    
            let bb_session = BBSession {
                domain: self.domain.to_string(),
                cookie_jar_dir: self.cookie_jar_dir.to_path_buf(),
                cookie_jar_path,
            };
            
            if bb_session.test_connection() {
                Ok(())
            } else {
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Initiated session connection test failed")))
            }
        }
    }

    pub fn test_connection(&self) -> bool {
        if self.cookie_jar_path.exists() {
            let mut easy = Easy::new();
            easy.url(&format!("https://{}/learn/api/public/v1/courses/_24810_1/contents", self.domain)).unwrap();
            easy.cookie_file(&self.cookie_jar_path).unwrap();
            easy.fail_on_error(true).unwrap();
            easy.perform().is_ok()
        } else {
            false
        }
    }

    pub fn download_file(&self, url: &str, out_path: &Path, overwrite: bool) -> Result<f64, Box<dyn std::error::Error>> {
        download::download_file(url, out_path, Some(&self.cookie_jar_path), overwrite)
    }

    pub fn download_and_unzip(&self, url: &str, out_path: &Path, overwrite: bool) -> Result<f64, Box<dyn std::error::Error>> {
        download::download_and_unzip(url, out_path, Some(&self.cookie_jar_path), overwrite)
    }

    fn download_course_contents_json(&self, course_id: &str, query_parameters: &[&str], out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        let mut url = format!("https://{}/learn/api/public/v1/courses/{}/contents",
            self.domain,
            course_id);
        
        if !query_parameters.is_empty() {
            url.extend(format!("?{}", query_parameters.join("&")).chars());
        }

        self.download_file(&url, out_path, true)
    }

    pub fn download_course_root_contents_json(&self, course_id: &str, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        self.download_course_contents_json(course_id, &[DEFAULT_FIELDS], out_path)
    }

    pub fn download_content_children_json(&self, course_id: &str, content_id: &str, query_parameters: &[&str], out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        let mut url = format!("https://{}/learn/api/public/v1/courses/{}/contents/{}/children",
            self.domain,
            course_id,
            content_id);
    
        if !query_parameters.is_empty() {
            url.extend(format!("?{}", query_parameters.join("&")).chars());
        }

        self.download_file(&url, out_path, true)
    }

    pub fn download_course_files_json(&self, course_id: &str, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        self.download_course_contents_json(course_id, &["contentHandler=resource/x-bb-file","recursive=true", DEFAULT_FIELDS], out_path)
    }

    pub fn download_course_documents_json(&self, course_id: &str, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        self.download_course_contents_json(course_id, &["contentHandler=resource/x-bb-document","recursive=true", DEFAULT_FIELDS], out_path)
    }

    pub fn download_course_assignments_json(&self, course_id: &str, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        self.download_course_contents_json(course_id, &["contentHandler=resource/x-bb-assignment","recursive=true", DEFAULT_FIELDS], out_path)
    }
    
    // Add query_parameters argument!
    pub fn download_course_announcements_json(&self, course_id: &str, query_parameters: &[&str], out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        // let fields = "id,title,contentHandler"; Alle egentlig interessante
        
        let mut url = format!("https://{}/learn/api/public/v1/courses/{}/announcements",
            self.domain,
            course_id);

        if !query_parameters.is_empty() {
            url.extend(format!("?{}", query_parameters.join("&")).chars());
        }

        self.download_file(&url, out_path, true)
    }
    
    pub fn download_content_attachments_json(&self, course_id: &str, content_id: &str, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        
        let url = format!("https://{}/learn/api/public/v1/courses/{}/contents/{}/attachments",
            self.domain,
            course_id,
            content_id);
    
        self.download_file(&url, out_path, true)
    }

    pub fn download_content_attachment(&self, course_id: &str, content_id: &str, attachment_id: &str, out_path: &Path, unzip: bool, overwrite: bool) -> Result<f64, Box<dyn std::error::Error>> {
        eprintln!("Downloading attachment {:?}", out_path.file_name().unwrap());

        let url = format!("https://{}/learn/api/public/v1/courses/{}/contents/{}/attachments/{}/download",
            self.domain,
            course_id,
            content_id,
            attachment_id);
        
        if unzip {
            self.download_and_unzip(&url, out_path, overwrite)
        } else {
            self.download_file(&url, out_path, overwrite)
        }
    }
    
    // pub fn download_course_assessment_questions_json(...)
}


impl std::convert::From<&BBSession> for json::JsonValue {
    fn from(session: &BBSession) -> json::JsonValue {
        json::object!{
            domain: session.domain.clone(),
            cookie_jar_dir: session.cookie_jar_dir.as_os_str().to_str().unwrap(),
        }
    }
}

impl std::convert::From<json::JsonValue> for BBSession {
    fn from(course: json::JsonValue) -> BBSession {
        BBSession::new(
            &course["domain"].to_string(), 
            Path::new(&course["cookie_jar_dir"].to_string())
        ).expect("Error creating new BBSession from JSON")
    }
}