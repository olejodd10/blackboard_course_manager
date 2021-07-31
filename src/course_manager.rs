use std::path::PathBuf;

pub mod course;
use course::Course;

struct CourseManager {
    courses: Vec<Box<dyn Course>>,
    base_dir: PathBuf,
}

impl CourseManager {
    fn add_course(&mut self, course: Box<dyn Course>) {
        self.courses.push(course);
    }

    fn remove_semester_course(&mut self, semester: &str, course_code: &str) {
        if let Some(index) = self.courses.iter().position(|c| {
            c.get_semester() == semester && c.get_course_code() == course_code
        }) {
            self.courses.remove(index);
        }
    }

    fn print_courses(&self) {
        for course in &self.courses {
            println!("{} {}", course.get_course_code(), course.get_semester());
        }
    }
}