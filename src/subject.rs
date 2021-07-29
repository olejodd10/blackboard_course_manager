use std::path::{PathBuf, Path};

mod fetch;
use fetch::fetch_file;

mod appointment_url;
use appointment_url::AppointmentUrlFormatString;

const UPPER_SEARCHED_APPOINTMENT_LIMIT: usize = 20;

pub trait Subject {

    fn available_appointments(&self) -> Vec<usize> {
        (1..=UPPER_SEARCHED_APPOINTMENT_LIMIT).take_while(|appointment_number| self.fetch_appointment(*appointment_number).is_ok()).collect()
    }

    //overwrite-argument!
    fn fetch_appointment(&self, appointment_number: usize) -> Result<(), Box<dyn std::error::Error>>;

    //overwrite-argument!
    fn fetch_appointments(&self, appointment_numbers: &[usize]) -> Result<(), Box<dyn std::error::Error>> {
        for appointment_number in appointment_numbers {
            let fetch_result = self.fetch_appointment(*appointment_number); 
            if !fetch_result.is_ok() {
                eprintln!("Fetching appointment number {} failed", appointment_number);
                return fetch_result;
            }
        }
        Ok(())
    }

    fn fetch_available_appointments(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.fetch_appointments(&self.available_appointments())
    }

    // fn submit(...);
}

pub struct WikiSubject {
    subject_code: String,
    semester: String,
    output_dir: PathBuf,
    appointment_url_format_string: AppointmentUrlFormatString,
}

impl WikiSubject {
    pub fn new(subject_code: &str, semester: &str, output_dir: &Path, appointment_url_format_string: Vec<String>) -> WikiSubject {
        WikiSubject {
            subject_code: String::from(subject_code),
            semester: String::from(semester),
            output_dir: PathBuf::from(output_dir),
            appointment_url_format_string: appointment_url::AppointmentUrlFormatString(appointment_url_format_string),
        }
    }
}

impl Subject for WikiSubject {    
    fn fetch_appointment(&self, appointment_number: usize) -> Result<(), Box<dyn std::error::Error>> {
        fetch_file(
            &self.appointment_url_format_string.appointment_url(appointment_number), 
            &self.output_dir.join(Path::new(&format!("{}_{}_{}.pdf", self.subject_code, self.semester, appointment_number)))
        )
    }
}


// struct BlackBoardSubject {
//     subject_code: String,
//     semester: String,
//     output_dir: String,
// }

// impl Subject for BlackBoardSubject {
//     fn fetch_appointment(appointment_number: usize) -> Result<(), Box<dyn std::error::Error>> {

//     }
// }