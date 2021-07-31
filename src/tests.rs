use std::path::{Path, PathBuf};

use super::course_manager::course::{self, Course};

#[test]
fn wiki_course_test() {
    let statistikk = super::course_manager::course::wiki_course::WikiCourse::new(
        "TMA4245", 
        "2021v", 
        Path::new("./"), 
        ["https://www.math.ntnu.no/emner/TMA4245/2021v/skriftlige_ovinger/inn","-oppg-b.pdf"]
        .iter().map(|s| String::from(*s)).collect(),
    );

    statistikk.download_available_appointments().unwrap();
}

#[test]
fn bb_file_fetch_parse_test() {
    let blackboard_course = course::blackboard_course::BBCourse {
        course_code: "TTK4105".to_string(),
        semester: "V21".to_string(),
        output_dir: PathBuf::from("./"),
        blackboard_course_id: "_24810_1".to_string(),
    };

    blackboard_course.fetch_file_attachments().unwrap();
}