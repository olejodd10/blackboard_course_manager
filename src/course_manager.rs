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