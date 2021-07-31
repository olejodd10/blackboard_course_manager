use crate::download::{download_file, download_and_unzip};
use std::path::PathBuf;

#[derive(Clone)]
struct BlackBoardFile {
    filename: String,
    mimetype: String,
    content_id: String,
    attachment_id: String,
}

struct BlackBoardCourse {
    course_code: String,
    semester: String,
    output_dir: PathBuf,
    blackboard_course_id: String,
}


impl BlackBoardCourse {
    fn appointment_is_nth_appointment(appointment: &BlackBoardFile, appointment_number: usize) -> bool {
        appointment.filename.find(&appointment_number.to_string()).is_some()
    }
    
    fn fetch_announcements(&self, limit: usize, offset: usize) {
        unimplemented!();
    }

    fn is_appointment(&self, file: &BlackBoardFile) -> bool {
        unimplemented!();
    }

    fn fetch_files(&self) -> Vec<BlackBoardFile> {
        unimplemented!();
    }

    fn fetch_appointments(&self) -> Vec<BlackBoardFile> {
        self.fetch_files().iter().filter(|file| self.is_appointment(file)).cloned().collect()
    }

}

impl super::Course for BlackBoardCourse {
    fn available_appointments(&self) -> Vec<usize> {
        unimplemented!();
    }

    fn download_appointment(&self, appointment_number: usize) -> Result<(), Box<dyn std::error::Error>> {
        let appointment = self.fetch_appointments()
            .into_iter()
            .find(|appointment| BlackBoardCourse::appointment_is_nth_appointment(appointment, appointment_number))
            .unwrap();
        
        let file_url = format!("https://ntnu.blackboard.com/learn/api/public/v1/courses/{}/contents/{}/attachments/{}/download",
            self.blackboard_course_id,
            appointment.content_id,
            appointment.attachment_id);
            
        if appointment.mimetype == "attribute/zip" {
            download_and_unzip(&file_url, &self.output_dir, None)?;
            Ok(())
        } else {
            let output_file_name = format!("{}_{}_{}.pdf", self.course_code, self.semester, appointment_number);
            download_file(&file_url, &self.output_dir.join(output_file_name), None)?;
            Ok(())
        }
    }
}
