use std::path::{Path, PathBuf};
use std::io::{BufRead, Write, Read};
use std::collections::HashMap;

pub mod blackboard_course;

use crate::course::Course;
use blackboard_course::BBCourse;
use blackboard_course::predicate_utils;
use blackboard_course::blackboard_session::{BBSession, input_utils::stdin_trimmed_line};
use blackboard_course::filename_utils::cookie_filename;
use blackboard_course::blackboard_definitions::BBContent;


use json;

pub struct BBCourseManager {
    courses: HashMap<String, BBCourse>,
    out_dir: PathBuf,
    work_dir: PathBuf,
}

impl BBCourseManager {
    pub fn new(out_dir: &Path, work_dir: &Path) -> BBCourseManager {
        std::fs::create_dir_all(out_dir).expect("Error creating BBCourseManager out_dir");
        std::fs::create_dir_all(work_dir).expect("Error creating BBCourseManager work_dir");
        let mut course_manager = BBCourseManager {
            courses: HashMap::new(),
            out_dir: out_dir.to_path_buf(),
            work_dir: work_dir.to_path_buf()
        };
        course_manager.load_courses();
        course_manager
    }

    pub fn register_course(&mut self) {
        println!("Please enter an alias for the new course:");
        let alias = stdin_trimmed_line();

        println!("Please enter the course code (format: TMA4100):");
        let course_code = stdin_trimmed_line();
        
        let semester = std::env::var("BBCM_SEMESTER").unwrap_or_else(|_| {
            println!("Please enter the semester (format: V2020, H2021):");
            stdin_trimmed_line()
        });
        
        let domain = std::env::var("BBCM_DOMAIN").unwrap_or_else(|_| {
            println!("Please enter BlackBoard domain the course belongs to (format: ntnu.blackboard.com):");
            stdin_trimmed_line()
        });

        // This dependency is weird, but it's important that different courses from the same domain use the same session. 
        let bb_session = blackboard_course::blackboard_session::BBSession::new(
            &domain, 
            &self.work_dir.join(format!("cookies/{}", cookie_filename(&domain)))
        ).expect("Error creating BBSession while registering course");

        println!("Please enter BlackBoard course id (format: _24810_1):");
        let id = stdin_trimmed_line();

        let bb_course = BBCourse::new(
            bb_session,
            &course_code,
            &semester,
            &alias,
            &self.out_dir.join(format!("{}\\{}", semester, alias)),
            &self.work_dir.join(format!("temp_{}", alias)),
            &id
        );

        self.add_course(bb_course);
    }

    fn add_course(&mut self, course: BBCourse) {
        self.courses.insert(course.get_alias().to_string(), course);
    }

    pub fn remove_course(&mut self, alias: &str) {
        if self.courses.remove(alias).is_none() {
            eprintln!("Unknown alias \"{}\". No course removed.", alias);
        }
    }

    pub fn remove_all_courses(&mut self) {
        self.courses.clear();
    }

    fn get_course(&self, alias: &str) -> Option<&BBCourse> {
        match self.courses.get(alias) {
            None => {
                eprintln!("No registered course with alias {}", alias);
                //...Do you want to register?
                None
            },
            other => other,
        }
    }

    pub fn view_courses(&self) {
        if self.courses.is_empty() {
            println!("No courses registered yet.");
        } else {
            for course in self.courses.values() {
                println!("{}: {} {}", course.get_alias(), course.get_course_code(), course.get_semester());
            }
        }
    }

    pub fn view_course_announcements(&self, alias: &str, limit: Option<usize>, offset: Option<usize>) -> Result<(), Box<dyn std::error::Error>> {
        self.get_course(alias).unwrap().view_course_announcements(limit, offset)
    }

    pub fn download_course_content_tree(
        &self, 
        alias: &str,
        title_substring: Option<String>, 
        filename_substring: Option<String>, 
        mimetype: Option<String>,
        unzip: bool, 
        overwrite: bool
    ) -> Result<f64, Box<dyn std::error::Error>> {
        if let Some(title_substring) = title_substring {
            if let Some(filename_substring) = filename_substring {
                if let Some(mimetype) = mimetype {
                    self.get_course(alias).unwrap().download_course_content_tree(
                        Some(&|content| predicate_utils::title_contains(content, &title_substring)),
                        Some(&|attachment| predicate_utils::filename_contains(attachment, &filename_substring)
                         && predicate_utils::mimetype_contains(attachment, &mimetype)),
                        unzip,
                        overwrite
                    )
                } else {
                    self.get_course(alias).unwrap().download_course_content_tree(
                        Some(&|content| predicate_utils::title_contains(content, &title_substring)),
                        Some(&|attachment| predicate_utils::filename_contains(attachment, &filename_substring)),
                        unzip,
                        overwrite
                    )
                }
            } else {
                if let Some(mimetype) = mimetype {
                    self.get_course(alias).unwrap().download_course_content_tree(
                        Some(&|content| predicate_utils::title_contains(content, &title_substring)),
                        Some(&|attachment| predicate_utils::mimetype_contains(attachment, &mimetype)),
                        unzip,
                        overwrite
                    )
                } else {
                    self.get_course(alias).unwrap().download_course_content_tree(
                        Some(&|content| predicate_utils::title_contains(content, &title_substring)),
                        None,
                        unzip,
                        overwrite
                    )
                }
            }
        } else {
            if let Some(filename_substring) = filename_substring {
                if let Some(mimetype) = mimetype {
                    self.get_course(alias).unwrap().download_course_content_tree(
                        None,
                        Some(&|attachment| predicate_utils::filename_contains(attachment, &filename_substring)
                         && predicate_utils::mimetype_contains(attachment, &mimetype)),
                        unzip,
                        overwrite
                    )
                } else {
                    self.get_course(alias).unwrap().download_course_content_tree(
                        None,
                        Some(&|attachment| predicate_utils::filename_contains(attachment, &filename_substring)),
                        unzip,
                        overwrite
                    )
                }
            } else {
                if let Some(mimetype) = mimetype {
                    self.get_course(alias).unwrap().download_course_content_tree(
                        None,
                        Some(&|attachment| predicate_utils::mimetype_contains(attachment, &mimetype)),
                        unzip,
                        overwrite
                    )
                } else {
                    self.get_course(alias).unwrap().download_course_content_tree(
                        None,
                        None,
                        unzip,
                        overwrite
                    )
                }
            }
        }
    }

    fn save_courses(&self) {
        let course_objects: Vec<json::JsonValue> = self.courses.iter().map(|(_, course)| {
            json::JsonValue::from(course)
        }).collect();
        let json_array = json::JsonValue::Array(course_objects); 
        let json_dump = json_array.pretty(4);
        let courses_file_path = self.work_dir.join("courses.json");
        if courses_file_path.exists() {
            std::fs::remove_file(&courses_file_path).expect("Error removing existing courses file");
        }
        let mut courses_file = std::fs::File::create(courses_file_path).expect("Error creating courses file path");
        courses_file.write_all(json_dump.as_bytes()).expect("Error writing to courses file");
    }

    fn load_courses(&mut self) {
        let courses_file_path = self.work_dir.join("courses.json");
        if courses_file_path.exists() {
            let mut courses_file = std::fs::File::open(courses_file_path).expect("Error opening courses file path");
            let mut json_string = String::new();
            courses_file.read_to_string(&mut json_string).expect("Error reading courses file");
            let courses_json = json::parse(&json_string).expect("Error parsing courses json");
            if let json::JsonValue::Array(courses) = courses_json {
                self.courses = courses.into_iter().map(|course| (course["alias"].to_string(), course.into())).collect();
            }
        }
    }

}

impl Drop for BBCourseManager {
    fn drop(&mut self) {
        self.save_courses();
    }
}
