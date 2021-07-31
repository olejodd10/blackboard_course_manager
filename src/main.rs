use std::path::Path;

mod download;
use download::download_file;

mod course_manager;

mod auth;

fn main() {
    // let statistikk = course::WikiCourse::new(
    //     "TMA4245", 
    //     "2021v", 
    //     Path::new("./"), 
    //     ["https://www.math.ntnu.no/emner/TMA4245/2021v/skriftlige_ovinger/inn","-oppg-b.pdf"]
    //     .iter().map(|s| String::from(*s)).collect(),
    // );

    // statistikk.fetch_available_appointments().unwrap();
    
    auth::get_authorization_code();

    // let file_url = "https://ntnu.blackboard.com/learn/api/v1/courses/_24006_1/announcements";
    // let out_path = Path::new("./test.json");
    // download_file(file_url, &out_path).unwrap();
}
