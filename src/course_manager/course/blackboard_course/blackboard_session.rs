use crate::download;
use std::path::{Path, PathBuf};

const DEFAULT_FIELDS: &str = "id,title,contentHandler";

#[derive(Debug)]
pub struct BBSession {
    pub domain: String,
    pub cookie_header: String,
}

impl BBSession {
    //Eventuelt kan disse ta inn BB-structs

    pub fn download_file(&self, url: &str, out_path: &Path, overwrite: bool) -> Result<f64, Box<dyn std::error::Error>> {
        download::download_file(&url, &out_path, Some(&[&self.cookie_header]), overwrite)
    }

    pub fn download_and_unzip(&self, url: &str, out_path: &Path, overwrite: bool) -> Result<f64, Box<dyn std::error::Error>> {
        download::download_and_unzip(&url, &out_path, Some(&[&self.cookie_header]), overwrite)
    }

    fn download_course_contents_json(&self, course_id: &str, content_handler: Option<&str>, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        let mut url = format!("https://{}/learn/api/public/v1/courses/{}/contents",
            self.domain,
            course_id);

        let mut query_arguments = Vec::new(); 
        
        //Default arguments:
        query_arguments.push("recursive=true".to_string());
        query_arguments.push(format!("fields={}", DEFAULT_FIELDS));

        if let Some(content_handler) = content_handler {
            query_arguments.push(format!("contentHandler={}", content_handler));
        }
    
        if !query_arguments.is_empty() {
            url.extend(format!("?{}", query_arguments.join("&")).chars());
        }

        self.download_file(&url, out_path, true)
    }

    pub fn download_course_files_json(&self, course_id: &str, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        self.download_course_contents_json(course_id, Some("resource/x-bb-file"), out_path)
    }

    pub fn download_course_documents_json(&self, course_id: &str, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        self.download_course_contents_json(course_id, Some("resource/x-bb-document"), out_path)
    }

    pub fn download_course_assignments_json(&self, course_id: &str, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        self.download_course_contents_json(course_id, Some("resource/x-bb-assignment"), out_path)
    }
    
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