use std::path::{PathBuf, Path};
use json;

pub mod blackboard_session;
pub mod blackboard_definitions;
pub mod appointment_evaluators;
use blackboard_definitions::{BBAttachment, BBContent, BBAnnouncement};

pub struct BBCourse<'a> {
    pub session: &'a blackboard_session::BBSession,
    pub course_code: String,
    pub semester: String,
    pub out_dir: PathBuf,
    pub temp_dir: PathBuf,
    pub id: String,
    pub appointment_evaluator: &'static dyn Fn(&BBAttachment) -> bool,
}


impl<'a> BBCourse<'a> {
    pub fn new(session: &'a blackboard_session::BBSession,
            course_code: String,
            semester: String,
            out_dir: PathBuf,
            id: String,
            appointment_evaluator: &'static dyn Fn(&BBAttachment) -> bool) -> BBCourse<'a> {
        let temp_dir = out_dir.clone().join("temp");
        std::fs::create_dir_all(&out_dir).expect("Error creating out folder");
        std::fs::create_dir_all(&temp_dir).expect("Error creating temp folder");
        BBCourse {
            session,
            course_code,
            semester,
            out_dir,
            temp_dir,
            id,
            appointment_evaluator, 
        }
    }

    fn attachment_is_appointment(&self, attachment: &BBAttachment) -> bool {
        (self.appointment_evaluator)(attachment)
    }

    fn appointment_is_nth_appointment(appointment: &BBAttachment, appointment_number: usize) -> bool {
        appointment.filename.find(&appointment_number.to_string()).is_some()
    }
    
    pub fn view_announcements(&self, limit: usize, offset: usize, width: usize) -> Result<(), Box<dyn std::error::Error>> {
        let announcements_json_filename = "announcements.json";
        let announcements_json_path = self.temp_dir.join(&announcements_json_filename);
        self.session.download_course_announcements_json(&self.id, limit, offset, &announcements_json_path)?;

        let course_announcements = BBAnnouncement::vec_from_json_results(&announcements_json_path)?;

        for announcement in course_announcements {
            println!("{}\nTITLE: {}\nCREATOR: {}\nCREATED: {}\nMODIFIED: {}\n{}\n{}\n",
                "*".repeat(width),
                announcement.title, 
                announcement.creator,
                announcement.created,
                announcement.modified,
                "-".repeat(width),
                html2text::from_read(announcement.body.as_bytes(), width), 
            );
        }

        Ok(())
    }

    //Overwrite-argument!
    pub fn download_appointments(&self) -> Result<(), Box<dyn std::error::Error>> {

        let files_json_filename = "files.json";
        let files_json_path = self.temp_dir.join(&files_json_filename);
        self.session.download_course_files_json(&self.id, &files_json_path)?;
        let course_files = BBContent::vec_from_json_results(&files_json_path)?;

        let documents_json_filename = "documents.json";
        let documents_json_path = self.temp_dir.join(&documents_json_filename);
        self.session.download_course_documents_json(&self.id, &documents_json_path)?;
        let mut course_documents = BBContent::vec_from_json_results(&documents_json_path)?;

        let assignments_json_filename = "assignments.json";
        let assignments_json_path = self.temp_dir.join(&assignments_json_filename);
        self.session.download_course_assignments_json(&self.id, &assignments_json_path)?;
        let mut course_assignments = BBContent::vec_from_json_results(&assignments_json_path)?;

        let mut course_contents = course_files;
        course_contents.append(&mut course_documents);
        course_contents.append(&mut course_assignments); 

        for content in course_contents {

            let attachments_json_filename = format!("{}_attachments.json", content.id);
            let attachments_json_path = self.temp_dir.join(&attachments_json_filename);
            self.session.download_content_attachments_json(&self.id, &content.id, &attachments_json_path)?;
            let content_attachments = BBAttachment::vec_from_json_results(&attachments_json_path)?;

            for attachment in content_attachments {
                if self.attachment_is_appointment(&attachment) {
                    // eprintln!("\n{:?} er en appointment+++++++++++++++", attachment);
                    self.session.download_content_attachment(&self.id, &content.id, &attachment.id, &self.out_dir.join(&attachment.filename))?;
                } else {
                    // eprintln!("\n{:?} er ikke en appointment-----------------", attachment);
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


impl<'a> Drop for BBCourse<'a> {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.temp_dir).expect("Error deleting temp_dir");
    }
}