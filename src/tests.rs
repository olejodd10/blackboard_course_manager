use std::path::{Path, PathBuf};

use super::course_manager::course::{self, Course};

const COOKIE_HEADER: &str = "Cookie: JSESSIONID=8F8748C7FF858E9C7FC508F09269CE98; _ga=GA1.2.1751170615.1627655908; _gid=GA1.2.1612839104.1627655908; OptanonConsent=isIABGlobal=false&datestamp=Sat+Jul+31+2021+13%3A49%3A37+GMT%2B0200+(Central+European+Summer+Time)&version=6.19.0&hosts=&consentId=5cb83fef-82ce-4462-967a-5141a08ed4a3&interactionCount=1&landingPath=NotLandingPage&groups=C0002%3A1%2CC0005%3A1%2CC0004%3A1%2CC0003%3A1%2CC0001%3A1&AwaitingReconsent=false; _gcl_au=1.1.312523879.1627660164; tc_ptidexpiry=1690732164415; tc_ptid=5zrWjwTY6IVjTm4uTjpxgp; _fbp=fb.1.1627660165377.42280692; nmstat=187f3168-e1dc-65d1-0ae2-ec874bbfbb4e; vid=e0ee7884-2782-4369-a161-a47944cd6102; _pf_id.55ea=e0ee7884-2782-4369-a161-a47944cd6102.1627660167.3.1627667840.1627665315.b6f462a9-2b98-4b7c-bd81-c416940b25b7; __ssds=2; __ssuzjsr2=a9be0cd8e; __uzmaj2=40bd6587-be37-41fc-a21a-64037592a39a; __uzmbj2=1627660167; __uzmcj2=187351623509; __uzmdj2=1627661664; BbRouter=expires:1627829967,id:8744D746D0833C109100A7C29CC0AC43,signature:b524d8d761c956817635634096ba34b200dac879fe3127c94d9acd9aa1509936,site:f4fe20be-98b0-4ecd-9039-d18ce2989292,timeout:10800,user:6375ce98b45a407e99ff9519c9064ad3,v:2,xsrf:5b6fe1cd-af5f-4a05-92c9-41cd119948bf; BbClientCalenderTimeZone=Europe/Oslo; web_client_cache_guid=3f9f1ffd-8ef6-43ef-98fd-4d8aa8ae5a5f; JSESSIONID=B3496AB5900DE8E5A57C8561192A1E6A; xythosdrive=0; AWSELB=0733E7E906D5AD63F9AA1C42FA3A042BA8E680ECF50B40EEF09874C04E54C4618ECBB1CC5B915D7ADFA8141FD35B311D99233F0C63452644BACE67DCAAF4C87CB46F66E3D9; AWSELBCORS=0733E7E906D5AD63F9AA1C42FA3A042BA8E680ECF50B40EEF09874C04E54C4618ECBB1CC5B915D7ADFA8141FD35B311D99233F0C63452644BACE67DCAAF4C87CB46F66E3D9";

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
fn regtek_appointments_test() {
    let session = course::blackboard_course::blackboard_session::BBSession {
        domain: "ntnu.blackboard.com".to_string(),
        cookie_header: COOKIE_HEADER.to_string(),
    };

    let regtek = course::blackboard_course::BBCourse {
        session: &session,
        course_code: "TTK4105".to_string(),
        semester: "V21".to_string(),
        out_dir: PathBuf::from("./ttk4105files/"),
        id: "_24810_1".to_string(),
        appointment_evaluator: &(|attachment| {
            course::blackboard_course::appointment_evaluators::mimetype_and_filename_substring(
                attachment, 
                "application/pdf", 
                "ving")
        }),
    };

    regtek.download_appointments().unwrap();
}

#[test]
fn cpp_appointments_test() {
    let session = course::blackboard_course::blackboard_session::BBSession {
        domain: "ntnu.blackboard.com".to_string(),
        cookie_header: COOKIE_HEADER.to_string(),
    };

    let cpp = course::blackboard_course::BBCourse {
        session: &session,
        course_code: "TDT4102".to_string(),
        semester: "V21".to_string(),
        out_dir: PathBuf::from("./tdt4102files/"),
        id: "_22729_1".to_string(),
        appointment_evaluator: &(|attachment| {
            course::blackboard_course::appointment_evaluators::mimetype_and_filename_substring(
                attachment, 
                "application/pdf", 
                "ving")
        }),
    };

    cpp.download_appointments().unwrap();
}

#[test]
fn bb_course_announcements_test() {
    let session = course::blackboard_course::blackboard_session::BBSession {
        domain: "ntnu.blackboard.com".to_string(),
        cookie_header: COOKIE_HEADER.to_string(),
    };

    let regtek = course::blackboard_course::BBCourse {
        session: &session,
        course_code: "TTK4105".to_string(),
        semester: "V21".to_string(),
        out_dir: PathBuf::from("./ttk4105files/"),
        id: "_24810_1".to_string(),
        appointment_evaluator: &(|attachment| {
            course::blackboard_course::appointment_evaluators::mimetype_and_filename_substring(
                attachment, 
                "application/pdf", 
                "ving")
        }),
    };

    regtek.view_announcements(2, 1, 70).unwrap();
}