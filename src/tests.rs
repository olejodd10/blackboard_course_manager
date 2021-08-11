use std::path::{Path, PathBuf};

use super::course_manager::course::{self, Course};
use course::blackboard_course::BBCourse;
use course::blackboard_course::blackboard_session::BBSession;
use course::blackboard_course::predicate_utils::small_file_mimetype;
use course::blackboard_course::predicate_utils::filename_substring;
use course::blackboard_course::predicate_utils::title_substring;


const COOKIE_FILE_PATH: &str = "cookies_test.txt";

#[test]
fn wiki_course_test() {
    let statistikk = super::course_manager::course::wiki_course::WikiCourse::new(
        "TMA4245", 
        "2021v", 
        "stat",
        Path::new(".\\output\\tma4245files\\"), 
        ["https://www.math.ntnu.no/emner/TMA4245/2021v/skriftlige_ovinger/inn","-oppg-b.pdf"]
        .iter().map(|s| String::from(*s)).collect(),
    );

    // statistikk.download_available_appointments().unwrap();
}

#[test]
fn regtek_appointments_test() {
    let session = BBSession {
        domain: "ntnu.blackboard.com".to_string(),
        cookie_jar_path: PathBuf::from(COOKIE_FILE_PATH)
    };

    let regtek = BBCourse::new(
        &session,
        "TTK4105",
        "V21",
        "regtek",
        Path::new(".\\output\\ttk4105files\\"),
        "_24810_1"
    );
    

    regtek.download_course_content_attachments(
        Some(&(
            |content| {
            course::blackboard_course::predicate_utils::title_substring(
                content, 
                "Øving ")
            }
        )), 
        None,
        true,
        false
    )
    .unwrap();
}

#[test]
fn cpp_test() {

    let session = BBSession {
        domain: "ntnu.blackboard.com".to_string(),
        cookie_jar_path: PathBuf::from(COOKIE_FILE_PATH)
    };
    let cpp = BBCourse::new(
        &session,
        "TDT4102",
        "V21",
        "cpp",
        Path::new(".\\output\\tdt4102files\\"),
        "_22729_1"
    );

    // cpp.download_course_content_attachments(
    //     Some(&(
    //         |content| {
    //         course::blackboard_course::predicate_utils::title_substring(
    //             content, 
    //             "Øvingstekst")
    //         }
    //     )), 
    //     None,
    //     true,
    //     false
    // )
    // .unwrap();

    cpp.download_course_content_tree(None, Some(&small_file_mimetype), false, false).unwrap();
}

#[test]
fn bb_course_announcements_test() {
    let session = BBSession {
        domain: "ntnu.blackboard.com".to_string(),
        cookie_jar_path: PathBuf::from(COOKIE_FILE_PATH)
    };
    let regtek = BBCourse::new(
        &session,
        "TTK4105",
        "V21",
        "regtek",
        Path::new(".\\output\\ttk4105files\\"),
        "_24810_1"
    );
    

    regtek.view_course_announcements(2, 0).unwrap();
}

#[test]
fn regtek_test() {
    let session = BBSession {
        domain: "ntnu.blackboard.com".to_string(),
        cookie_jar_path: PathBuf::from(COOKIE_FILE_PATH)
    };
    let regtek = BBCourse::new(
        &session,
        "TTK4105",
        "V21",
        "regtek",
        Path::new(".\\output\\ttk4105files\\"),
        "_24810_1"
    );
    
    
    regtek.view_course_announcements(2, 0).unwrap();
    
    regtek.view_course_content(None).unwrap();

    regtek.download_course_content_attachments(
        Some(&(
            |content| {
            course::blackboard_course::predicate_utils::title_substring(
                content, 
                "Øving")
            }
        )), 
        None,
        false,
        true
    )
    .unwrap();

    regtek.download_course_content_tree(None, Some(&small_file_mimetype), false, false).unwrap();    
}


#[test]
fn tilpdat_test() {
    let session = BBSession {
        domain: "ntnu.blackboard.com".to_string(),
        cookie_jar_path: PathBuf::from(COOKIE_FILE_PATH)
    };
    let tilpdat = BBCourse::new(
        &session,
        "TTK4235",
        "V21",
        "tilpdat",
        Path::new(".\\output\\ttk4235files\\"),
        "_24561_1"
    );
    
    // tilpdat.view_course_announcements(2, 0).unwrap();
    
    // tilpdat.view_course_content(None).unwrap();
    // tilpdat.view_course_attachments(Some(&|attachment| filename_substring(attachment, "2020"))).unwrap();
    // tilpdat.view_course_content(Some(&|content| title_substring(content, "2020"))).unwrap();

    // tilpdat.download_course_content_attachments(
    //     Some(&(
    //         |content| {
    //         course::blackboard_course::predicate_utils::title_substring(
    //             content, 
    //             "Øving")
    //         }
    //     )), 
    //     None,
    //     false,
    //     true
    // )
    // .unwrap();

    tilpdat.download_course_content_tree(None, Some(&small_file_mimetype), true, false).unwrap();    
}

#[test]
fn test_long_filenames() {
    let session = BBSession {
        domain: "ntnu.blackboard.com".to_string(),
        cookie_jar_path: PathBuf::from(COOKIE_FILE_PATH)
    };
    // https://ntnu.blackboard.com/learn/api/public/v1/courses/_24810_1/contents/_1245662_1/attachments/_3062124_1/download
    session.download_content_attachment(
        "_24810_1", 
        "_1245662_1", 
        "_3062124_1", 
        Path::new(".\\output\\ttk4105files\\content_tree\\Undervisningsmateriell\\Video_av_forelesninger_vår_2020\\Videoinnholdsfortegnelse_2020\\aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.pdf"), 
        false,
        false
    ).unwrap();


}


#[test]
fn test_bb_session_initiate() {
    use std::io::Write;
    use curl::easy::Easy;
    let session = BBSession::initiate_bb_session("ntnu.blackboard.com",Path::new(COOKIE_FILE_PATH)).unwrap();
    
    session.download_course_announcements_json("_24810_1", 3, 0, Path::new("HALLO.txt")).unwrap();
}