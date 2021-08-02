use std::path::{Path, PathBuf};

use super::course_manager::course::{self, Course};
use course::blackboard_course::BBCourse;
use course::blackboard_course::blackboard_session::BBSession;


const COOKIE_HEADER: &str = "Cookie: JSESSIONID=14A28AAF0C158B87CBC92B0CF220710A; _ga=GA1.2.1751170615.1627655908; OptanonConsent=isIABGlobal=false&datestamp=Sat+Jul+31+2021+13%3A49%3A37+GMT%2B0200+(Central+European+Summer+Time)&version=6.19.0&hosts=&consentId=5cb83fef-82ce-4462-967a-5141a08ed4a3&interactionCount=1&landingPath=NotLandingPage&groups=C0002%3A1%2CC0005%3A1%2CC0004%3A1%2CC0003%3A1%2CC0001%3A1&AwaitingReconsent=false; _gcl_au=1.1.312523879.1627660164; tc_ptidexpiry=1690732164415; tc_ptid=5zrWjwTY6IVjTm4uTjpxgp; _fbp=fb.1.1627660165377.42280692; nmstat=187f3168-e1dc-65d1-0ae2-ec874bbfbb4e; vid=e0ee7884-2782-4369-a161-a47944cd6102; _pf_id.55ea=e0ee7884-2782-4369-a161-a47944cd6102.1627660167.3.1627667840.1627665315.b6f462a9-2b98-4b7c-bd81-c416940b25b7; __ssds=2; __ssuzjsr2=a9be0cd8e; __uzmaj2=40bd6587-be37-41fc-a21a-64037592a39a; __uzmbj2=1627660167; __uzmcj2=187351623509; __uzmdj2=1627661664; BbRouter=expires:1627941123,id:630C4BB3F738BA58D42653ADF453FA29,signature:d1f209cba59a4286443cb82e533b4b3c51bd0ee8e2afec4865b80cc00dac0503,site:f4fe20be-98b0-4ecd-9039-d18ce2989292,timeout:10800,user:6375ce98b45a407e99ff9519c9064ad3,v:2,xsrf:ebf459c3-7e54-47ae-8d67-fe5f74c6e9f4; BbClientCalenderTimeZone=Europe/Oslo; web_client_cache_guid=3f9f1ffd-8ef6-43ef-98fd-4d8aa8ae5a5f; xythosdrive=0; JSESSIONID=66D9404E1ACE21C34CFE5684A8F194AC; AWSELB=0733E7E906D5AD63F9AA1C42FA3A042BA8E680ECF50B40EEF09874C04E54C4618ECBB1CC5B915D7ADFA8141FD35B311D99233F0C6334F007D6202E9E5255D6E1B93BE64A50; AWSELBCORS=0733E7E906D5AD63F9AA1C42FA3A042BA8E680ECF50B40EEF09874C04E54C4618ECBB1CC5B915D7ADFA8141FD35B311D99233F0C6334F007D6202E9E5255D6E1B93BE64A50";

#[test]
fn wiki_course_test() {
    let statistikk = super::course_manager::course::wiki_course::WikiCourse::new(
        "TMA4245", 
        "2021v", 
        "stat",
        Path::new("./output/tma4245files/"), 
        ["https://www.math.ntnu.no/emner/TMA4245/2021v/skriftlige_ovinger/inn","-oppg-b.pdf"]
        .iter().map(|s| String::from(*s)).collect(),
    );

    // statistikk.download_available_appointments().unwrap();
}

#[test]
fn regtek_appointments_test() {
    let session = BBSession {
        domain: "ntnu.blackboard.com".to_string(),
        cookie_header: COOKIE_HEADER.to_string()
    };

    let regtek = BBCourse::new(
        &session,
        "TTK4105",
        "V21",
        "regtek",
        Path::new("./output/ttk4105files/"),
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
fn cpp_appointments_test() {

    let session = BBSession {
        domain: "ntnu.blackboard.com".to_string(),
        cookie_header: COOKIE_HEADER.to_string()
    };
    let cpp = BBCourse::new(
        &session,
        "TDT4102",
        "V21",
        "cpp",
        Path::new("./output/tdt4102files/"),
        "_22729_1"
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
        true,
        false
    )
    .unwrap();
}

#[test]
fn bb_course_announcements_test() {
    let session = BBSession {
        domain: "ntnu.blackboard.com".to_string(),
        cookie_header: COOKIE_HEADER.to_string()
    };
    let regtek = BBCourse::new(
        &session,
        "TTK4105",
        "V21",
        "regtek",
        Path::new("./output/ttk4105files/"),
        "_24810_1"
    );
    

    regtek.view_course_announcements(2, 0).unwrap();
}

#[test]
fn regtek_test() {
    let session = BBSession {
        domain: "ntnu.blackboard.com".to_string(),
        cookie_header: COOKIE_HEADER.to_string()
    };
    let regtek = BBCourse::new(
        &session,
        "TTK4105",
        "V21",
        "regtek",
        Path::new("./output/ttk4105files/"),
        "_24810_1"
    );
    
    
    regtek.view_course_announcements(2, 0).unwrap();
    
    regtek.view_course_content().unwrap();

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

    let downloaded_amount = regtek.download_course_content_tree(false, false).unwrap();    
}


#[test]
fn tilpdat_test() {
    let session = BBSession {
        domain: "ntnu.blackboard.com".to_string(),
        cookie_header: COOKIE_HEADER.to_string()
    };
    let tilpdat = BBCourse::new(
        &session,
        "TTK4235",
        "V21",
        "tilpdat",
        Path::new("./output/ttk4235files/"),
        "_24561_1"
    );
    
    tilpdat.view_course_announcements(2, 0).unwrap();
    
    tilpdat.view_course_content().unwrap();

    tilpdat.download_course_content_attachments(
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

    let downloaded_amount = tilpdat.download_course_content_tree(false, false).unwrap();    
}
