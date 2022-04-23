use std::path::{PathBuf, Path};

mod input_utils;
pub mod filename_utils;
pub mod bb_content;
pub mod bb_announcement;
pub mod predicate_utils;
use super::BBCourseManager;
use bb_content::BBContent;
use bb_announcement::BBAnnouncement;
use bb_content::bb_attachment::BBAttachment;
use input_utils::stdin_trimmed_line;

pub struct BBCourse<'a> {
    manager: &'a BBCourseManager,
    pub course_code: String,
    pub semester: String,
    pub alias: String,
    out_dir: PathBuf,
    id: String,
    pub last_tree_download: String,
}

impl<'a> BBCourse<'a> {
    pub fn new(
        manager: &'a BBCourseManager, 
        course_code: &str,
        semester: &str,
        alias: &str,
        out_dir: &Path,
        id: &str,
        last_tree_download: &str
    ) -> BBCourse<'a> {
        std::fs::create_dir_all(&out_dir).expect("Error creating base folder"); // This is a bit ugly. An init function would be better.
        BBCourse {
            manager,
            course_code: course_code.to_string(),
            semester: semester.to_string(),
            alias: alias.to_string(),
            out_dir: out_dir.to_path_buf(),
            id: id.to_string(),
            last_tree_download: last_tree_download.to_string(),
        }
    }

    fn ids_and_names_from_json_results(json: Vec<u8>) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
        let json_string = std::string::String::from_utf8(json)?;
        let parsed_json = json::parse(&json_string)?;

        Ok(parsed_json["results"].members().map(|member| {
            (member["id"].to_string(), member["name"].to_string())
        }).collect())
    }

    pub fn register(manager: &BBCourseManager) -> BBCourse {
        println!("Please enter the course code (format: TMA4100):");
        let course_code = stdin_trimmed_line();
        
        let semester = std::env::var("BBCM_SEMESTER").unwrap_or_else(|_| {
            println!("Please enter the semester (format: 2020_V, 2021_H):"); // This matches the NTNU courseId convention
            stdin_trimmed_line()
        });

        let courses_json = manager.download_courses_json(&[&format!("courseId={}%{}", course_code, semester)]).expect("Error: Could not download courses json");
        let (id, name) = BBCourse::ids_and_names_from_json_results(courses_json).expect("Error: Could not parse courses json").first().expect("Error: No matching course found").to_owned();

        println!("Found course \"{}\".\nPlease enter an alias for the new course:", name);
        let alias = stdin_trimmed_line();

        BBCourse::new(
            manager,
            &course_code,
            &semester,
            &alias,
            &manager.out_dir.join(format!("bbcm_{}\\{}", semester, alias)),
            &id,
            ""
        )
    }
        
    fn download_course_contents_json(&self, query_parameters: &[&str]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut url = format!("https://{}/learn/api/public/v1/courses/{}/contents",
            self.manager.session.domain,
            self.id);
        
        if !query_parameters.is_empty() {
            url.extend(format!("?{}", query_parameters.join("&")).chars());
        }

        self.manager.session.download_bytes(&url)
    }

    fn download_course_root_contents_json(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.download_course_contents_json(&[bb_content::BBContent::DEFAULT_FIELDS])
    }
    
    fn download_course_announcements_json(&self, query_parameters: &[&str]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // let fields = "id,title,contentHandler"; Alle egentlig interessante
        
        let mut url = format!("https://{}/learn/api/public/v1/courses/{}/announcements",
            self.manager.session.domain,
            self.id);

        if !query_parameters.is_empty() {
            url.extend(format!("?{}", query_parameters.join("&")).chars());
        }

        self.manager.session.download_bytes(&url)
    }

    fn get_course_root_content(&self) -> Result<Vec<BBContent>, Box<dyn std::error::Error>> {
        let json = self.download_course_root_contents_json()?;
        BBContent::vec_from_json_results(json, self)
    }

    pub fn download_course_content_tree(
        &self, 
        content_predicate: Option<&dyn Fn(&BBContent) -> bool>, 
        attachment_predicate: Option<&dyn Fn(&BBAttachment) -> bool>,
        overwrite: bool
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut threads = Vec::new();
        // std::fs::create_dir_all(&self.tree_dir).expect("Error creating tree dir"); //Hvorfor klagde ikke denne når jeg hadde "?"?
        for content in self.get_course_root_content()? {
            content.download_children(content_predicate, attachment_predicate, &self.out_dir, overwrite, &mut threads)?;
        }
        let total_download_size = threads.into_iter().map(|t| t.join().expect("Failed to join thread")).sum();
        Ok(total_download_size)
    }

    //Announcements
    fn get_course_announcements(&self, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<BBAnnouncement>, Box<dyn std::error::Error>> {

        let mut query_parameters = Vec::new();
        if let Some(limit) = limit {
            query_parameters.push(format!("limit={}", limit));
        }
        if let Some(offset) = offset {
            query_parameters.push(format!("offset={}", offset));
        }

        let borrowed_query_parameters: Vec<&str> = query_parameters.iter().map(|s| s.as_str()).collect();

        let json = self.download_course_announcements_json(&borrowed_query_parameters[..])?;
        BBAnnouncement::vec_from_json_results(json)
    }
    
    pub fn view_course_announcements(&self, limit: Option<usize>, offset: Option<usize>) -> Result<(), Box<dyn std::error::Error>> {
        let announcements = self.get_course_announcements(limit, offset)?;
        if announcements.is_empty() {
            println!("No announcements found.")
        } else {
            for announcement in self.get_course_announcements(limit, offset)? {
                announcement.view_with_name(self.manager);
            }
        }
        Ok(())
    }

    pub fn view(&self) {
        println!("{}: {} {}", self.alias, self.course_code, self.semester);
    }
    
    // pub fn download_course_assessment_questions_json(...)
}

// impl<'a> Drop for BBCourse<'a> {
//     fn drop(&mut self) {
//         if self.temp_dir.exists() {
//             std::fs::remove_dir_all(&self.temp_dir).expect("Error deleting temp_dir");
//         }
//     }
// }

impl<'a> std::convert::From<&BBCourse<'a>> for json::JsonValue {
    fn from(course: &BBCourse) -> json::JsonValue {
        json::object!{
            course_code: course.course_code.clone(),
            semester: course.semester.clone(),
            alias: course.alias.clone(),
            out_dir: course.out_dir.as_os_str().to_str().unwrap(),
            id: course.id.clone(),
            last_tree_download: course.last_tree_download.clone(),
        }
    }
}