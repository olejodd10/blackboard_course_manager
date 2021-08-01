use std::path::{PathBuf, Path};
use json;

pub mod blackboard_session;
pub mod blackboard_definitions;
use blackboard_definitions::{BBAttachment, BBContent};

#[derive(Debug)]
pub struct BBCourse<'a> {
    pub session: &'a blackboard_session::BBSession,
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

    pub fn download_appointments(&self) -> Result<(), Box<dyn std::error::Error>> {

        let files_json_filename = format!("{}_{}_files.json", self.course_code, self.semester);
        let files_json_path = self.out_dir.join(&files_json_filename);
        self.session.download_course_files_json(&self.id, &files_json_path)?;
        let course_files = BBContent::vec_from_json_results(&files_json_path)?;

        let documents_json_filename = format!("{}_{}_documents.json", self.course_code, self.semester);
        let documents_json_path = self.out_dir.join(&documents_json_filename);
        self.session.download_course_files_json(&self.id, &documents_json_path)?;
        let course_documents = BBContent::vec_from_json_results(&documents_json_path)?;

        for content in course_files.iter().chain(course_documents.iter()) {
            let attachments_json_filename = format!("{}_{}_{}_attachments.json", self.course_code, self.semester, content.id);
            let attachments_json_path = self.out_dir.join(&attachments_json_filename);
            self.session.download_content_attachments_json(&self.id, &content.id, &attachments_json_path)?;
            let content_attachments = BBAttachment::vec_from_json_results(&attachments_json_path)?;

            for attachment in content_attachments {
                if BBCourse::attachment_is_appointment(&attachment) {
                    eprintln!("{:?} er en appointment", attachment);
                    self.session.download_content_attachment(&self.id, &content.id, &attachment.id, &self.out_dir.join(&attachment.filename))?;
                } 
            }
        }

        Ok(())
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
