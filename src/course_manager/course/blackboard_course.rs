use crate::download::{download_file, download_and_unzip};
use std::path::{PathBuf, Path};
use json;

#[derive(Clone, Debug)]
pub struct BBAttachment {
    attachment_id: String,
    filename: String,
    content_id: String,
    mimetype: String,
}

impl BBAttachment {
    pub fn download(&self, out_path: &Path, access_token: Option<&str>) -> Result<f64, Box<dyn std::error::Error>> {
        let domain = "ntnu.blackboard.com"; // FIX
        let course_id = "_22729_1"; // FIX
        let url = format!("https://{}/learn/api/public/v1/courses/{}/contents/{}/attachments/{}/download",
            domain, 
            course_id, 
            self.content_id, 
            self.attachment_id);

        if self.mimetype == "application/zip" {
            download_file(&url, out_path, access_token)
        } else {
            download_and_unzip(&url, out_path, access_token)
        }
    }
}

// #[derive(Clone)]
// struct BBDocument {
//  ...
// }

// pub struct BBAnnouncement {
//     ...
// }


pub struct BBCourse {
    pub course_code: String,
    pub semester: String,
    pub output_dir: PathBuf,
    pub blackboard_course_id: String,
}


impl BBCourse {
    fn is_appointment(&self, attachment: &BBAttachment) -> bool {
        attachment.mimetype == "application/pdf" && attachment.filename.find("ving").is_some()
    }

    pub fn appointment_is_nth_appointment(appointment: &BBAttachment, appointment_number: usize) -> bool {
        appointment.filename.find(&appointment_number.to_string()).is_some()
    }
    
    pub fn fetch_announcements(&self, limit: usize, offset: usize) {
        unimplemented!();
    }

    fn fetch_attachments(&self, content_id: &str) -> Result<Vec<BBAttachment>, Box<dyn std::error::Error>> {
        let domain = "ntnu.blackboard.com";
        
        let url = format!("https://{}/learn/api/public/v1/courses/{}/contents/{}/attachments",
            domain,
            self.blackboard_course_id,
            content_id);

        let json_filename = format!("{}_attachments.json", content_id);
        let json_path = self.output_dir.join(&json_filename);

        // download_file(&url, &json_path, None)?;

        let json_string = std::fs::read_to_string(&json_path)?;
        let parsed_json = json::parse(&json_string)?;

        let fetched_files = parsed_json["results"].members().map(|member| {
            BBAttachment {
                attachment_id: member["id"].to_string(),
                filename: member["fileName"].to_string(),
                content_id: content_id.to_string(),
                mimetype: member["mimeType"].to_string(),                
            }
        }).collect();

        Ok(fetched_files)
    }

    pub fn fetch_file_attachments(&self) -> Result<Vec<BBAttachment>, Box<dyn std::error::Error>> {
        let domain = "ntnu.blackboard.com";
        let content_handler_filter = "resource/x-bb-file";
        let fields = "id,title,contentHandler"; //Trenger kanskje ikke contentHandler utenom filtreringen
        let recursive = "true";

        let url = format!("https://{}/learn/api/public/v1/courses/{}/contents?recursive={}&fields={}&contentHandler={}",
            domain,
            self.blackboard_course_id,
            recursive,
            fields,
            content_handler_filter);
        
        let json_filename = format!("{}_files.json", self.course_code);
        let json_path = self.output_dir.join(&json_filename);
        
        // download_file(&url, &json_path, None)?;

        let json_string = std::fs::read_to_string(&json_path)?;
        let parsed_json = json::parse(&json_string)?;

        let mut fetched_files = Vec::new();
        for member in parsed_json["results"].members() {
            let content_id = member["id"].to_string();
            if content_id != "_1214272_1" {continue;}
            fetched_files.append(&mut self.fetch_attachments(&content_id)?);
        };

        eprintln!("FETCHED_FILES_____________\n{:?}", fetched_files);

        Ok(fetched_files)
    }

    // fn fetch_documents(&self) -> Vec<BBDocument> {
    //     let content_handler_filter = "resource/x-bb-file";
    //     let fields = "id,title,contentHandler"; //Trenger kanskje ikke contentHandler utenom filtreringen
        
    //     unimplemented!();
    // }

    pub fn fetch_appointments(&self) -> Result<Vec<BBAttachment>, Box<dyn std::error::Error>> {
        Ok(
            self.fetch_file_attachments()?.iter().filter(|attachment| self.is_appointment(attachment)).cloned().collect()
        )
    }

}

// impl super::Course for BBCourse {
//     fn available_appointments(&self) -> Vec<usize> {
//         unimplemented!();
//     }

//     fn download_appointment(&self, appointment_number: usize) -> Result<(), Box<dyn std::error::Error>> {
//         let appointment = self.fetch_appointments()
//             .into_iter()
//             .find(|appointment| BBCourse::appointment_is_nth_appointment(appointment, appointment_number))
//             .unwrap();
        
//         let file_url = format!("https://ntnu.blackboard.com/learn/api/public/v1/courses/{}/contents/{}/attachments/{}/download",
//             self.blackboard_course_id,
//             appointment.content_id,
//             appointment.attachment_id);
            
//         if appointment.mimetype == "attribute/zip" {
//             download_and_unzip(&file_url, &self.output_dir, None)?;
//             Ok(())
//         } else {
//             let output_file_name = format!("{}_{}_{}.pdf", self.course_code, self.semester, appointment_number);
//             download_file(&file_url, &self.output_dir.join(output_file_name), None)?;
//             Ok(())
//         }
//     }
// }
