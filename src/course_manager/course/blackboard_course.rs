use std::path::{PathBuf, Path};
use json;

mod blackboard_session;
mod blackboard_definitions;
use blackboard_definitions::{BBAttachment, BBContent};

#[derive(Debug)]
pub struct BBCourse<'a> {
    session: &'a blackboard_session::BBSession,
    pub course_files: Vec<BBContent>,
    pub course_documents: Vec<BBContent>,
    pub course_code: String,
    pub semester: String,
    pub out_dir: PathBuf,
    pub id: String,
}


impl<'a> BBCourse<'a> {
    fn attachment_is_appointment(attachment: &BBAttachment) -> bool {
        attachment.mimetype == "application/pdf" && attachment.filename.find("ving").is_some()
    }

    fn appointment_is_nth_appointment(appointment: &BBAttachment, appointment_number: usize) -> bool {
        appointment.filename.find(&appointment_number.to_string()).is_some()
    }
    
    // pub fn fetch_announcements(&self, limit: usize, offset: usize) -> Result<Vec<BBAnnouncement>, Box<dyn std::error::Error>> {
    //     unimplemented!();
    // }

    fn fetch_course_files(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let session = self.session;

        let json_filename = format!("{}_files.json", self.id);
        let json_path = self.out_dir.join(&json_filename);

        session.download_course_files_json(&self.id, &json_path)?;

        let json_string = std::fs::read_to_string(&json_path)?;
        let parsed_json = json::parse(&json_string)?;

        self.course_files = parsed_json["results"].members().map(|member| {
            BBContent {
                attachments: Vec::new(),
                id: member["id"].to_string(),
                title: member["title"].to_string(),
            }
        }).collect();

        Ok(())
    }

    fn fetch_course_file_attachments(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for file in &mut self.course_files {
            file.fetch_attachments(&self)?
        }
        Ok(())
    }

    fn fetch_course_documents(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let session = self.session;

        let json_filename = format!("{}_documents.json", self.id);
        let json_path = self.out_dir.join(&json_filename);

        session.download_course_documents_json(&self.id, &json_path)?;

        let json_string = std::fs::read_to_string(&json_path)?;
        let parsed_json = json::parse(&json_string)?;

        self.course_documents = parsed_json["results"].members().map(|member| {
            BBContent {
                attachments: Vec::new(),
                id: member["id"].to_string(),
                title: member["title"].to_string(),
            }
        }).collect();

        Ok(())
    }

    fn fetch_course_document_attachments(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for document in &mut self.course_documents {
            document.fetch_attachments(&self)?
        }
        Ok(())
    }

    pub fn get_course_appointments(&self) -> Result<Vec<&BBAttachment>, Box<dyn std::error::Error>> {

        let mut appointments = Vec::new();
        for content in self.course_files.iter().chain(self.course_documents.iter()) {
            for attachment in &content.attachments {
                if BBCourse::attachment_is_appointment(attachment) {
                    appointments.push(attachment);
                }
            }
        }

        Ok(appointments)
    }
}

// impl super::Course for BBCourse {
//     fn get_available_appointments(&self) -> Vec<usize> {
//         let appointments = self.get_course_appointments();
//         (1..20).filter(|appointment_number| {
//             appointments.iter().any(|appointment| BBCourse::appointment_is_nth_appointment(*appointment, appointment_number))
//         }).collect()
//     }

//     fn download_appointment(&self, appointment_number: usize) -> Result<(), Box<dyn std::error::Error>> {
//         let appointment = self.get_course_appointments()
//             .into_iter()
//             .find(|appointment| BBCourse::appointment_is_nth_appointment(appointment, appointment_number))
//             .unwrap();
        
//         let file_url = format!("https://ntnu.blackboard.com/learn/api/public/v1/courses/{}/contents/{}/attachments/{}/download",
//             self.id,
//             appointment.content_id,
//             appointment.attachment_id);
            
//         if appointment.mimetype == "attribute/zip" {
//             self.session.download_file(&file_url, &self.out_dir)?;
//             Ok(())
//         } else {
//             let output_file_name = format!("{}_{}_{}.pdf", self.course_code, self.semester, appointment_number);
//             self.session.download_file(&file_url, &self.out_dir.join(output_file_name))?;
//             Ok(())
//         }
//     }
// }
