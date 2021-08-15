
pub trait Course {
    //fn get_institution(&self) -> &str;
    
    fn get_alias(&self) -> &str;

    fn set_alias(&mut self, new_alias: &str);

    fn get_course_code(&self) -> &str;

    fn get_semester(&self) -> &str;

    // fn download_everything(&self, overwrite: usize);

    // fn submit(...);
}
