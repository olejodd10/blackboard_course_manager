use std::path::{Path, PathBuf};
use std::io::BufRead;

pub mod course;
mod input_utils;

use course::Course;
use course::blackboard_course::BBCourse;
use course::blackboard_course::blackboard_session::BBSession;
use input_utils::stdin_trimmed_line;

pub struct CourseManager {
    courses: Vec<Box<dyn Course>>,
    out_dir: PathBuf,
    work_dir: PathBuf,
}

impl CourseManager {
    pub fn new(out_dir: &Path, work_dir: &Path) -> CourseManager {
        std::fs::create_dir_all(out_dir).expect("Error creating CourseManager out_dir");
        std::fs::create_dir_all(work_dir).expect("Error creating CourseManager work_dir");
        CourseManager {
            courses: Vec::new(),
            out_dir: out_dir.to_path_buf(),
            work_dir: work_dir.to_path_buf()
        }
    }

    fn add_course(&mut self, course: Box<dyn Course>) {
        self.courses.push(course);
    }

    pub fn remove_semester_course(&mut self, alias: &str) {
        if let Some(index) = self.courses.iter().position(|c| {
            c.get_alias() == alias
        }) {
            self.courses.remove(index);
        } else {
            eprintln!("Unknown alias \"{}\"", alias);
        }
    }

    pub fn print_courses(&self) {
        for course in &self.courses {
            println!("{}: {} {}", course.get_alias(), course.get_course_code(), course.get_semester());
        }
    }

    fn create_bb_session(&mut self, domain: &str) -> Result<BBSession, Box<dyn std::error::Error>> {
        BBSession::new(domain, &self.work_dir.join("cookies"))
    }

    pub fn register_course(&mut self) {
        println!("Please enter an alias for the new course:");
        let alias = stdin_trimmed_line();

        println!("Please enter the course code (format: TMA4100):");
        let course_code = stdin_trimmed_line();
        
        println!("Please enter the semester (format: V2020, H2021):");
        let semester = stdin_trimmed_line();

        println!("Is the course a BlackBoard course? y/n");
        let is_bb_course = stdin_trimmed_line();

        if is_bb_course == "y" {
            println!("Please enter BlackBoard domain the course belongs to (format: ntnu.blackboard.com):");
            let domain = stdin_trimmed_line();

            let bb_session = self.create_bb_session(&domain).expect("Error creating BBSession while registering course");

            println!("Please enter BlackBoard course id (format: _24810_1):");
            let id = stdin_trimmed_line();

            let bb_course = BBCourse::new(
                bb_session,
                &course_code,
                &semester,
                &alias,
                &self.out_dir.join(format!("{}/{}", semester, course_code)),
                &id
            );

            self.add_course(Box::new(bb_course));
        } else {
            unimplemented!();
        }
    }

    //fn save

    // fn load

}