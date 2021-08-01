pub mod wiki_course;
pub mod blackboard_course;

pub trait Course {
    //fn get_institution(&self) -> &str;
    
    // fn get_alias(&self) -> &str;

    fn get_course_code(&self) -> &str;

    fn get_semester(&self) -> &str;

    fn get_available_appointments(&self) -> Vec<usize>;

    //overwrite-argument!
    fn download_appointment(&self, appointment_number: usize) -> Result<(), Box<dyn std::error::Error>>;

    //overwrite-argument!
    fn download_appointments(&self, appointment_numbers: &[usize]) -> Result<(), Box<dyn std::error::Error>> {
        for appointment_number in appointment_numbers {
            let download_result = self.download_appointment(*appointment_number); 
            if !download_result.is_ok() {
                eprintln!("Downloading appointment number {} failed", appointment_number);
                return download_result;
            }
        }
        Ok(())
    }

    fn download_available_appointments(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.download_appointments(&self.get_available_appointments())
    }

    // fn submit(...);
}
