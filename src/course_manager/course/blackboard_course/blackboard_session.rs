use crate::download;
use std::path::{Path, PathBuf};

const DEFAULT_FIELDS: &str = "id,title,contentHandler";

#[derive(Debug)]
pub struct BBSession {
    domain: String,
    cookie_header: String,
}

impl BBSession {
    //Eventuelt kan disse ta inn BB-structs

    pub fn download_file(&self, url: &str, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        download::download_file(&url, &out_path, Some(&[&self.cookie_header]))
    }
    
    pub fn download_course_announcements_json(&self, course_id: &str, limit: usize, offset: usize, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        // let fields = "id,title,contentHandler"; Alle egentlig interessante
        
        let url = format!("https://{}/learn/api/public/v1/courses/{}/announcements",
            self.domain,
            course_id);
                
        self.download_file(&url, &out_path)
    }

    pub fn download_course_files_json(&self, course_id: &str, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        let content_handler_filter = "resource/x-bb-file";
        let recursive = "true";

        let url = format!("https://{}/learn/api/public/v1/courses/{}/contents?recursive={}&fields={}&contentHandler={}",
            self.domain,
            course_id,
            recursive,
            DEFAULT_FIELDS,
            content_handler_filter);
        
        self.download_file(&url, &out_path)
    }

    pub fn download_course_documents_json(&self, course_id: &str, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        let content_handler_filter = "resource/x-bb-document";
        let recursive = "true";

        let url = format!("https://{}/learn/api/public/v1/courses/{}/contents?recursive={}&fields={}&contentHandler={}",
            self.domain,
            course_id,
            recursive,
            DEFAULT_FIELDS,
            content_handler_filter);
        
        self.download_file(&url, &out_path)
    }
    
    pub fn download_content_attachments_json(&self, course_id: &str, content_id: &str, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        
        let url = format!("https://{}/learn/api/public/v1/courses/{}/contents/{}/attachments",
            self.domain,
            course_id,
            content_id);
    
        self.download_file(&url, &out_path)
    }

    pub fn download_content_attachment(&self, course_id: &str, content_id: &str, attachment_id: &str, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        
        let url = format!("https://{}/learn/api/public/v1/courses/{}/contents/{}/attachments/{}/download",
            self.domain,
            course_id,
            content_id,
            attachment_id);

        self.download_file(&url, &out_path)
    }
    
    // pub fn download_course_assessment_questions_json(...)
}