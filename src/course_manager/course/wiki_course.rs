use std::path::{PathBuf, Path};

mod appointment_url;
use appointment_url::AppointmentUrlFormatString;

use crate::download::download_file;

const LOWER_SEARCHED_APPOINTMENT_LIMIT: usize = 1;
const UPPER_SEARCHED_APPOINTMENT_LIMIT: usize = 20;

pub struct WikiCourse {
    course_code: String,
    semester: String,
    out_dir: PathBuf,
    appointment_url_format_string: AppointmentUrlFormatString,
}

impl WikiCourse {
    pub fn new(course_code: &str, semester: &str, out_dir: &Path, appointment_url_format_string: Vec<String>) -> WikiCourse {
        std::fs::create_dir_all(&out_dir).expect("Error creating out folder");
        WikiCourse {
            course_code: String::from(course_code),
            semester: String::from(semester),
            out_dir: PathBuf::from(out_dir),
            appointment_url_format_string: appointment_url::AppointmentUrlFormatString(appointment_url_format_string),
        }
    }
}

impl super::Course for WikiCourse {
    fn get_course_code(&self) -> &str {
        &self.course_code
    }

    fn get_semester(&self) -> &str {
        &self.semester
    }

    fn get_available_appointments(&self) -> Vec<usize> {
        (LOWER_SEARCHED_APPOINTMENT_LIMIT..=UPPER_SEARCHED_APPOINTMENT_LIMIT).take_while(|appointment_number| self.download_appointment(*appointment_number).is_ok()).collect()
    }
    
    fn download_appointment(&self, appointment_number: usize) -> Result<(), Box<dyn std::error::Error>> {
        const VALID_APPOINTMENT_SIZE_LIMIT: f64 = 10000.0; 
        let appointment_path = self.out_dir.join(&format!("{}_{}_{}.pdf", self.course_code, self.semester, appointment_number));
        match download_file(
            &self.appointment_url_format_string.appointment_url(appointment_number), 
            &appointment_path,
            None,
            false
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