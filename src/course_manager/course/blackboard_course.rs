use std::path::{PathBuf, Path};
use json;

pub mod blackboard_session;
pub mod blackboard_definitions;
pub mod predicate_utils;
use blackboard_definitions::{BBAttachment, BBContent, BBAnnouncement, BBContentHandler};

pub struct BBCourse<'a> {
    pub session: &'a blackboard_session::BBSession,
    pub course_code: String,
    pub semester: String,
    out_dir: PathBuf,
    files_dir: PathBuf,
    temp_dir: PathBuf,
    tree_dir: PathBuf,
    pub id: String,
}


impl<'a> BBCourse<'a> {
    pub fn new(
            session: &'a blackboard_session::BBSession,
            course_code: String,
            semester: String,
            out_dir: PathBuf,
            id: String
        ) -> BBCourse<'a> {
        let temp_dir = out_dir.join("temp");
        let files_dir = out_dir.join("downloaded_files");
        let tree_dir = out_dir.join("content_tree");
        std::fs::create_dir_all(&out_dir).expect("Error creating out folder");
        std::fs::create_dir_all(&temp_dir).expect("Error creating temp folder");
        BBCourse {
            session,
            course_code,
            semester,
            out_dir,
            files_dir,
            temp_dir,
            tree_dir,
            // announcements_dir, ...
            id,
        }
    }

    fn appointment_is_nth_appointment(appointment: &BBAttachment, appointment_number: usize) -> bool {
        appointment.filename.find(&appointment_number.to_string()).is_some()
    }

    // Everything it takes to create the course content tree
    fn get_course_root_content(&self) -> Result<Vec<BBContent>, Box<dyn std::error::Error>> {
        let root_content_json_filename = "root_content.json";
        let root_content_json_path = self.temp_dir.join(&root_content_json_filename);
        self.session.download_course_files_json(&self.id, &root_content_json_path)?;
        BBContent::vec_from_json_results(&root_content_json_path)
    }

    fn get_content_children(&self, content: &BBContent) -> Result<Vec<BBContent>, Box<dyn std::error::Error>> {
        let content_children_json_filename = format!("{}_children.json", content.title);
        let content_children_json_path = self.temp_dir.join(&content_children_json_filename);
        self.session.download_course_files_json(&self.id, &content_children_json_path)?;
        BBContent::vec_from_json_results(&content_children_json_path)
    }

    pub fn download_course_content_tree(&self, unzip: bool, overwrite: bool) -> Result<f64, Box<dyn std::error::Error>> {
        let mut total_download_size = 0.0;
        std::fs::create_dir(&self.tree_dir).expect("Error creating tree dir"); //Hvorfor klagde ikke denne når jeg hadde "?"?
        for content in self.get_course_root_content()? {
            total_download_size += self.download_children(&content, &self.tree_dir, unzip, overwrite)?;
        }
        Ok(total_download_size)
    }

    fn download_children(&self, content: &BBContent, out_path: &Path, unzip: bool, overwrite: bool) -> Result<f64, Box<dyn std::error::Error>> {
        match content.content_handler {
            BBContentHandler::XBBFile | BBContentHandler::XBBDocument | BBContentHandler::XBBAssignment => self.download_content_attachments(content, None, &out_path.join(&content.title), unzip, overwrite),
            BBContentHandler::XBBFolder => {
                let child_path = out_path.join(&content.title);
                let mut total_download_size = 0.0;
                std::fs::create_dir(&child_path).expect("Error creating child dir"); //Hvorfor klagde ikke denne når jeg hadde "?"?
                for child in self.get_content_children(content)? {
                    total_download_size += self.download_children(&child, &child_path, unzip, overwrite)?;
                }
                Ok(total_download_size)
            },
            _ => Ok(0.0)
        }
    }

    //Announcements
    pub fn get_course_announcements(&self, limit: usize, offset: usize) -> Result<Vec<BBAnnouncement>, Box<dyn std::error::Error>> {
        let announcements_json_filename = "announcements.json";
        let announcements_json_path = self.temp_dir.join(&announcements_json_filename);
        self.session.download_course_announcements_json(&self.id, limit, offset, &announcements_json_path)?;
        BBAnnouncement::vec_from_json_results(&announcements_json_path)
    }
    
    pub fn view_course_announcements(&self, limit: usize, offset: usize) -> Result<(), Box<dyn std::error::Error>> {
        for announcement in self.get_course_announcements(limit, offset)? {
            announcement.view();
        }
        Ok(())
    }

    // Course content, to get specific files (not tree)
    pub fn get_course_files(&self) -> Result<Vec<BBContent>, Box<dyn std::error::Error>> {
        let files_json_filename = "files.json";
        let files_json_path = self.temp_dir.join(&files_json_filename);
        self.session.download_course_files_json(&self.id, &files_json_path)?;
        BBContent::vec_from_json_results(&files_json_path)
    }

    pub fn get_course_documents(&self) -> Result<Vec<BBContent>, Box<dyn std::error::Error>> {
        let documents_json_filename = "documents.json";
        let documents_json_path = self.temp_dir.join(&documents_json_filename);
        self.session.download_course_documents_json(&self.id, &documents_json_path)?;
        BBContent::vec_from_json_results(&documents_json_path)
    }

    pub fn get_course_assignments(&self) -> Result<Vec<BBContent>, Box<dyn std::error::Error>> {
        let assignments_json_filename = "assignments.json";
        let assignments_json_path = self.temp_dir.join(&assignments_json_filename);
        self.session.download_course_assignments_json(&self.id, &assignments_json_path)?;
        BBContent::vec_from_json_results(&assignments_json_path)
    }

    pub fn get_course_content(&self) -> Result<Vec<BBContent>, Box<dyn std::error::Error>> {
        let mut course_contents = Vec::new();
        course_contents.append(&mut self.get_course_files()?);
        course_contents.append(&mut self.get_course_documents()?);
        course_contents.append(&mut self.get_course_assignments()?);
        Ok(course_contents)
    }

    pub fn view_course_content(&self) -> Result<(), Box<dyn std::error::Error>> {
        for content in self.get_course_content()? {
            content.view();
        }
        Ok(())
    }

    // Attachments
    pub fn get_content_attachments(&self, content: &BBContent) -> Result<Vec<BBAttachment>, Box<dyn std::error::Error>> {
        let attachments_json_filename = format!("{}_attachments.json", content.title);
        let attachments_json_path = self.temp_dir.join(&attachments_json_filename);
        self.session.download_content_attachments_json(&self.id, &content.id, &attachments_json_path)?;
        BBAttachment::vec_from_json_results(&attachments_json_path)
    }

    // Download attachments satisfying predicate, for specified content instance
    pub fn download_content_attachments(
        &self, 
        content: &BBContent, 
        attachment_predicate: Option<&'static dyn Fn(&BBAttachment) -> bool>,
        out_path: &Path,
        unzip: bool,
        overwrite: bool
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let content_attachments = self.get_content_attachments(content)?;
        let mut total_download_size = 0.0;
        if let Some(attachment_predicate) = attachment_predicate {
            for attachment in content_attachments.into_iter().filter(|attachment| attachment_predicate(attachment)) {
                let unzip = unzip && attachment.is_zip(); // Only unzip if unzip flag set, and file is zipped
                total_download_size += self.session.download_content_attachment(&self.id, &content.id, &attachment.id, &out_path.join(&attachment.filename), unzip, overwrite)?;
            }
        } else {
            for attachment in content_attachments {
                let unzip = unzip && attachment.is_zip(); // Only unzip if unzip flag set, and file is zipped
                total_download_size += self.session.download_content_attachment(&self.id, &content.id, &attachment.id, &out_path.join(&attachment.filename), unzip, overwrite)?;
            }
        }
        Ok(total_download_size)
    }

    // Download all attachments in course meeting predicates
    pub fn download_course_content_attachments(
        &self, 
        content_predicate: Option<&'static dyn Fn(&BBContent) -> bool>, 
        attachment_predicate: Option<&'static dyn Fn(&BBAttachment) -> bool>,
        unzip: bool,
        overwrite: bool
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let course_content = self.get_course_content()?;
        let mut total_download_size = 0.0;
        if let Some(content_predicate) = content_predicate {
            for content in course_content.into_iter().filter(|content| content_predicate(content)) {
                total_download_size += self.download_content_attachments(&content, attachment_predicate, &self.files_dir, unzip, overwrite)?;
            }
        } else {
            for content in course_content {
                total_download_size += self.download_content_attachments(&content, attachment_predicate, &self.files_dir, unzip, overwrite)?;
            }
        }
        Ok(total_download_size)
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