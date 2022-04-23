use std::path::{Path, PathBuf};
use std::io::{Read, Write};

pub mod bb_course;
pub mod cookie_session;
pub mod utils;

use cookie_session::CookieSession;
use bb_course::BBCourse;
use crate::utils::filename_utils::cookie_filename;

// Remove this unnecessary struct. Or rename.
pub struct BBCourseManager {
    session: CookieSession,
    out_dir: PathBuf,
    work_dir: PathBuf,
}

impl<'a> BBCourseManager {
    pub fn new(domain: &str, out_dir: &Path, work_dir: &Path) -> BBCourseManager {
        std::fs::create_dir_all(out_dir).expect("Error creating BBCourseManager out_dir");
        std::fs::create_dir_all(work_dir).expect("Error creating BBCourseManager work_dir");
        let cookie_jar_path = work_dir.join(format!("cookies\\{}", cookie_filename(domain)));
        BBCourseManager {
            session: CookieSession::new(domain, cookie_jar_path.as_ref()).unwrap(),
            out_dir: out_dir.to_path_buf(),
            work_dir: work_dir.to_path_buf()
        }
    }

    pub fn load_courses(&'a self) -> Vec<BBCourse<'a>> {
        let json_path = self.work_dir.join("courses.json");
        let mut json_string = String::new();
        if json_path.exists() {
            let mut courses_file = std::fs::File::open(&json_path).expect("Error opening courses json");
            courses_file.read_to_string(&mut json_string).expect("Error reading courses file");
        } else {
            json_string = String::from("[]");
        };
        let courses_json = json::parse(&json_string).expect("Error parsing courses json");
        if let json::JsonValue::Array(courses) = courses_json {
            courses.into_iter().map(|course| {
                BBCourse::new(
                    self,
                    &course["course_code"].to_string(),
                    &course["semester"].to_string(),
                    &course["alias"].to_string(),
                    Path::new(&course["out_dir"].to_string()),
                    &course["id"].to_string(),
                    &course["last_tree_download"].to_string(),
                )
            }).collect()
        } else {
            panic!("Unknown json format in courses file.");
        }
    }
    
    pub fn save_courses(&self, courses: &[BBCourse]) {
        let out_path = self.work_dir.join("courses.json");
        let course_objects: Vec<json::JsonValue> = courses.iter().map(|course| {
            json::JsonValue::from(course)
        }).collect();
        let json_array = json::JsonValue::Array(course_objects); 
        let json_dump = json_array.pretty(4);
        if out_path.exists() {
            std::fs::remove_file(&out_path).expect("Error removing existing courses file");
        }
        let mut courses_file = std::fs::File::create(out_path).expect("Error creating courses file path");
        courses_file.write_all(json_dump.as_bytes()).expect("Error writing to courses file");
    }

    pub fn download_courses_json(&self, query_parameters: &[&str]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut url = format!("https://{}/learn/api/public/v3/courses",
            self.session.domain);
        
        if !query_parameters.is_empty() {
            url.extend(format!("?{}", query_parameters.join("&")).chars());
        }

        self.session.download_bytes(&url)
    }
}
