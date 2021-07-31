use std::path::{PathBuf, Path};

mod appointment_url;
use appointment_url::AppointmentUrlFormatString;

use crate::download::download_file;

const LOWER_SEARCHED_APPOINTMENT_LIMIT: usize = 1;
const UPPER_SEARCHED_APPOINTMENT_LIMIT: usize = 20;

pub struct WikiCourse {
    course_code: String,
    semester: String,
    output_dir: PathBuf,
    appointment_url_format_string: AppointmentUrlFormatString,
}

impl WikiCourse {
    pub fn new(course_code: &str, semester: &str, output_dir: &Path, appointment_url_format_string: Vec<String>) -> WikiCourse {
        WikiCourse {
            course_code: String::from(course_code),
            semester: String::from(semester),
            output_dir: PathBuf::from(output_dir),
            appointment_url_format_string: appointment_url::AppointmentUrlFormatString(appointment_url_format_string),
        }
    }
}

impl super::Course for WikiCourse {    
    fn available_appointments(&self) -> Vec<usize> {
        (LOWER_SEARCHED_APPOINTMENT_LIMIT..=UPPER_SEARCHED_APPOINTMENT_LIMIT).take_while(|appointment_number| self.download_appointment(*appointment_number).is_ok()).collect()
    }
    
    fn download_appointment(&self, appointment_number: usize) -> Result<(), Box<dyn std::error::Error>> {
        const VALID_APPOINTMENT_SIZE_LIMIT: f64 = 10000.0; 
        let appointment_path = &format!("{}_{}_{}.pdf", self.course_code, self.semester, appointment_number);
        match download_file(
            &self.appointment_url_format_string.appointment_url(appointment_number), 
            &self.output_dir.join(Path::new(appointment_path)),
            None
        ) {
            Ok(download_size) if download_size < VALID_APPOINTMENT_SIZE_LIMIT => {
                std::fs::remove_file(appointment_path)?;
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "File too small to be valid")))
            },
            _ => {},
        }
        Ok(())
    }
}