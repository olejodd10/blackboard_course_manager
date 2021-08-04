use std::path::{Path, PathBuf};

pub mod course;
use course::Course;
use course::blackboard_course::blackboard_session::BBSession;

struct CourseManager {
    courses: Vec<Box<dyn Course>>,
    base_dir: PathBuf,
    bb_session: Option<BBSession>,
}

impl CourseManager {
    fn new(base_dir: &Path) -> CourseManager {
        CourseManager {
            courses: Vec::new(),
            base_dir: PathBuf::from(base_dir),
            bb_session: None,
        }
    }

    fn initiate_bb_session(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // https://tech.saigonist.com/b/code/how-login-any-website-using-curl-command-line-or-shell-script.html
        
        
        //http://innsida.ntnu.no/blackboard
        // https://idp.feide.no/simplesaml/module.php/feide/login?org=ntnu.no&AuthState=_2b5f061e41d8bc488329885f23b46a3c57a2550dcf%3Ahttps%3A%2F%2Fidp.feide.no%2Fsimplesaml%2Fsaml2%2Fidp%2FSSOService.php%3Fspentityid%3Dhttp%253A%252F%252Fadfs.ntnu.no%252Fadfs%252Fservices%252Ftrust%26RelayState%3D71d64f68-ca97-4120-a164-cb6b2e4a0bc3%26cookieTime%3D1628027373
        
        
        unimplemented!();
        Ok(())
    }

    fn add_course(&mut self, course: Box<dyn Course>) {
        self.courses.push(course);
    }

    fn remove_semester_course(&mut self, alias: &str) {
        if let Some(index) = self.courses.iter().position(|c| {
            c.get_alias() == alias
        }) {
            self.courses.remove(index);
        } else {
            eprintln!("Unknown alias \"{}\"", alias);
        }
    }

    fn print_courses(&self) {
        for course in &self.courses {
            println!("\"{}\": {} {}", course.get_alias(), course.get_course_code(), course.get_semester());
        }
    }

    fn register_course(&mut self) {
        unimplemented!();
    }

}