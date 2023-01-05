// https://rust-cli.github.io/book/index.html
use std::path::{PathBuf, Path};
use std::collections::HashMap;
use std::io::{Read, Write};
use structopt::StructOpt;

mod utils;
mod bbcm;
mod bb_course;
mod bb_session;

//OBS!! Merk at std::error::Error er en trait, mens std::io::Error er en struct!!
use bb_course::BBCourse;
use bb_session::BBSession;
use utils::{
    filename_utils::cookie_filename,
    time_utils::utc_now,
    input_utils::stdin_trimmed_line,
};
use bbcm::Bbcm;

pub fn load_courses(json_path: &Path) -> Vec<BBCourse> {
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

pub fn save_courses(courses: &[BBCourse], out_path: &Path) {
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

fn main() {
    let domain = std::env::var("BBCM_DOMAIN").unwrap_or_else(|_| {
        println!("Please enter the blackboard domain (format: <institution>.blackboard.com):"); // This matches the NTNU courseId convention
        let value = stdin_trimmed_line();
        std::env::set_var("BBCM_DOMAIN", &value);
        value
    });
    let out_dir = std::env::var("BBCM_OUT_DIR").map(|s| PathBuf::from(s)).unwrap_or_else(|_| {
        println!("Please enter the desired output directory (format: /path/to/directory):"); // This matches the NTNU courseId convention
        let value = stdin_trimmed_line();
        std::env::set_var("BBCM_OUT_DIR", &value);
        PathBuf::from(value)
    });
    let work_dir = std::env::var("BBCM_WORK_DIR").map(|val| PathBuf::from(&val)).unwrap_or_else(|_| std::env::temp_dir().join("bbcm_work"));
    std::fs::create_dir_all(&out_dir).expect("Error creating BBCourseManager out_dir");
    std::fs::create_dir_all(&work_dir).expect("Error creating BBCourseManager work_dir");
    let cookie_jar_path = work_dir.join(cookie_filename(&domain));
    let session = BBSession::new(&domain, &cookie_jar_path).expect("Error creating session");
    let courses_json_path = work_dir.join("courses.json");
    let mut courses: HashMap<String, BBCourse> = load_courses(&courses_json_path).into_iter().map(|course| (course.alias.clone(), course)).collect();

    match Bbcm::from_args() {
        Bbcm::Register => {
            let course = BBCourse::register(&session, &out_dir);
            courses.insert(course.alias.clone(), course);
        },

        Bbcm::Courses => {
            if courses.is_empty() {
                println!("No courses registered yet.");
            } else {
                for course in courses.values() {
                    course.view();
                }
            }
        },

        Bbcm::Tree {
            course_alias,
            overwrite,
        } => {
            if let Some(course) = courses.get_mut(&course_alias) {
                if let Ok(download_size) = course.download_course_content_tree(&session, overwrite) {
                    println!("Downloaded a total of {:.1} MB.", download_size/1000000.0);
                    course.last_tree_download = utc_now();
                } 
            } else {
                eprintln!("Course with alias {} not found.", course_alias);
            }
        },

        Bbcm::Trees {
            overwrite,
        } => {
            for (alias, course) in &mut courses {
                println!("Downloading tree for {}.", alias);
                if let Ok(download_size) = course.download_course_content_tree(&session, overwrite) {
                    println!("Downloaded a total of {:.1} MB.", download_size/1000000.0);
                    course.last_tree_download = utc_now();
                } 
            } 
        },

        Bbcm::Announcements {
            course_alias,
            limit,
            offset,
        } => {
            if let Some(course) = courses.get(&course_alias) {
                course.view_course_announcements(&session, limit, offset).unwrap();
            } else {
                eprintln!("Course with alias {} not found.", course_alias);
            }
        },

        Bbcm::Gradebooks {
            past,
        } => {
            for (alias, course) in &courses {
                println!("Viewing gradebook columns for {}.", alias);
                course.view_course_gradebook(&session, past).unwrap();
            } 
        },

        Bbcm::Remove {
            course_alias,
        } => {
            if courses.remove(&course_alias).is_none() {
                eprintln!("Course with alias {} not found.", course_alias);
            }
        },

        Bbcm::Reset => {
            courses.clear();
        }
    }

    save_courses(&courses.into_iter().map(|t| t.1).collect::<Vec<BBCourse>>(), &courses_json_path);
}
