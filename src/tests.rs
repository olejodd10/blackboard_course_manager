use std::path::{Path, PathBuf};

use super::course_manager::course::{self, Course};

const COOKIE_HEADER: &str = "Cookie: _ga=GA1.2.1751170615.1627655908; OptanonConsent=isIABGlobal=false&datestamp=Sat+Jul+31+2021+13%3A49%3A37+GMT%2B0200+(Central+European+Summer+Time)&version=6.19.0&hosts=&consentId=5cb83fef-82ce-4462-967a-5141a08ed4a3&interactionCount=1&landingPath=NotLandingPage&groups=C0002%3A1%2CC0005%3A1%2CC0004%3A1%2CC0003%3A1%2CC0001%3A1&AwaitingReconsent=false; _gcl_au=1.1.312523879.1627660164; tc_ptidexpiry=1690732164415; tc_ptid=5zrWjwTY6IVjTm4uTjpxgp; _fbp=fb.1.1627660165377.42280692; nmstat=187f3168-e1dc-65d1-0ae2-ec874bbfbb4e; vid=e0ee7884-2782-4369-a161-a47944cd6102; _pf_id.55ea=e0ee7884-2782-4369-a161-a47944cd6102.1627660167.3.1627667840.1627665315.b6f462a9-2b98-4b7c-bd81-c416940b25b7; __ssds=2; __ssuzjsr2=a9be0cd8e; __uzmaj2=40bd6587-be37-41fc-a21a-64037592a39a; __uzmbj2=1627660167; __uzmcj2=187351623509; __uzmdj2=1627661664; BbRouter=expires:1627857961,id:3D8F0AB6A21A395898AFAA4AD4922842,signature:2c90a2f66bbf509afdfb80dcf23da7b9831eeaf23dbc113f01ec083dc1fe419f,site:f4fe20be-98b0-4ecd-9039-d18ce2989292,timeout:10800,user:6375ce98b45a407e99ff9519c9064ad3,v:2,xsrf:25dc437a-0ee1-4fe3-ac75-021cab081210; BbClientCalenderTimeZone=Europe/Oslo; web_client_cache_guid=3f9f1ffd-8ef6-43ef-98fd-4d8aa8ae5a5f; JSESSIONID=B3496AB5900DE8E5A57C8561192A1E6A; xythosdrive=0; AWSELB=0733E7E906D5AD63F9AA1C42FA3A042BA8E680ECF50B40EEF09874C04E54C4618ECBB1CC5B56084689EE6FF7C1AAD701054A09855D3CE8B31A6B05D51228FD16B9B997D5EB; AWSELBCORS=0733E7E906D5AD63F9AA1C42FA3A042BA8E680ECF50B40EEF09874C04E54C4618ECBB1CC5B56084689EE6FF7C1AAD701054A09855D3CE8B31A6B05D51228FD16B9B997D5EB";

#[test]
fn wiki_course_test() {
    let statistikk = super::course_manager::course::wiki_course::WikiCourse::new(
        "TMA4245", 
        "2021v", 
        Path::new("./output/tma4245files/"), 
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

    let regtek = course::blackboard_course::BBCourse::new(
        &session,
        "TTK4105".to_string(),
        "V21".to_string(),
        PathBuf::from("./output/ttk4105files/"),
        "_24810_1".to_string()
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
        false
    )
    .unwrap();
}

#[test]
fn cpp_appointments_test() {
    let session = course::blackboard_course::blackboard_session::BBSession {
        domain: "ntnu.blackboard.com".to_string(),
        cookie_header: COOKIE_HEADER.to_string(),
    };

    let cpp = course::blackboard_course::BBCourse::new(
        &session,
        "TDT4102".to_string(),
        "V21".to_string(),
        PathBuf::from("./output/tdt4102files/"),
        "_22729_1".to_string()
    );

    cpp.download_course_content_attachments(
        Some(&(
            |content| {
            course::blackboard_course::predicate_utils::title_substring(
                content, 
                "Øvingstekst")
            }
        )), 
        None,
        false
    )
    .unwrap();}

#[test]
fn bb_course_announcements_test() {
    let session = course::blackboard_course::blackboard_session::BBSession {
        domain: "ntnu.blackboard.com".to_string(),
        cookie_header: COOKIE_HEADER.to_string(),
    };

    let regtek = course::blackboard_course::BBCourse::new(
        &session,
        "TTK4105".to_string(),
        "V21".to_string(),
        PathBuf::from("./output/ttk4105files/"),
        "_24810_1".to_string()
    );

    regtek.view_course_announcements(2, 0).unwrap();
}

#[test]
fn test_view_regtek_course_content() {
    let session = course::blackboard_course::blackboard_session::BBSession {
        domain: "ntnu.blackboard.com".to_string(),
        cookie_header: COOKIE_HEADER.to_string(),
    };

    let regtek = course::blackboard_course::BBCourse::new(
        &session,
        "TTK4105".to_string(),
        "V21".to_string(),
        PathBuf::from("./output/ttk4105files/"),
        "_24810_1".to_string()
    );

    regtek.view_course_content().unwrap();
}