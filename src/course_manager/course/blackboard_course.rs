use std::path::{PathBuf, Path};
use json;

mod blackboard_session;
mod blackboard_definitions;
use blackboard_definitions::{BBAttachment, BBContent};

#[derive(Debug)]
pub struct BBCourse<'a> {
    session: &'a blackboard_session::BBSession,
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

    fn fetch_course_files(&self) -> Result<Vec<BBContent>, Box<dyn std::error::Error>> {
        let session = self.session;

        let json_filename = format!("{}_files.json", self.id);
        let json_path = self.out_dir.join(&json_filename);

        session.download_course_files_json(&self.id, &json_path)?;

        let json_string = std::fs::read_to_string(&json_path)?;
        let parsed_json = json::parse(&json_string)?;

        let course_files = parsed_json["results"].members().map(|member| {
            BBContent {
                course: &self,
                id: member["id"].to_string(),
                title: member["title"].to_string(),
            }
        }).collect();

        Ok(course_files)
    }

    fn fetch_course_documents(&self) -> Result<Vec<BBContent>, Box<dyn std::error::Error>> {
        let session = self.session;

        let json_filename = format!("{}_documents.json", self.id);
        let json_path = self.out_dir.join(&json_filename);

        session.download_course_documents_json(&self.id, &json_path)?;

        let json_string = std::fs::read_to_string(&json_path)?;
        let parsed_json = json::parse(&json_string)?;

        let course_documents = parsed_json["results"].members().map(|member| {
            BBContent {
                course: &self,
                id: member["id"].to_string(),
                title: member["title"].to_string(),
            }
        }).collect();

        Ok(course_documents)
    }

    // fn fetch_documents(&self) -> Vec<BBDocument> {
    //     let content_handler_filter = "resource/x-bb-file";
    //     let fields = "id,title,contentHandler"; //Trenger kanskje ikke contentHandler utenom filtreringen
        
    //     unimplemented!();
    // }

    pub fn fetch_course_appointments(&self) -> Result<Vec<BBAttachment>, Box<dyn std::error::Error>> {
        let mut course_contents_of_interest = Vec::new();
        course_contents_of_interest.append(&mut self.fetch_course_files()?);
        course_contents_of_interest.append(&mut self.fetch_course_documents()?);
        
        let mut appointments = Vec::new();
        for content in course_contents_of_interest {
            for attachment in content.fetch_attachments()? {
                if BBCourse::attachment_is_appointment(&attachment) {
                    appointments.push(attachment);
                }
            }
        }

        Ok(appointments)
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
//             self.course_id,
//             appointment.content_id,
//             appointment.attachment_id);
            
//         if appointment.mimetype == "attribute/zip" {
//             download_and_unzip(&file_url, &self.out_dir, None)?;
//             Ok(())
//         } else {
//             let output_file_name = format!("{}_{}_{}.pdf", self.course_code, self.semester, appointment_number);
//             download_file(&file_url, &self.out_dir.join(output_file_name), None)?;
//             Ok(())
//         }
//     }
// }
