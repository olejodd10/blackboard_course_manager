use std::path::{PathBuf, Path};
use std::io::Write;

pub mod filename_utils;
pub mod blackboard_session;
pub mod blackboard_content;
pub mod blackboard_announcement;
pub mod predicate_utils;
use blackboard_content::{BBContent, BBContentHandler};
use blackboard_announcement::BBAnnouncement;
use blackboard_content::blackboard_attachment::BBAttachment;
use filename_utils::{valid_dir_name, valid_filename};

pub struct BBCourse {
    session: blackboard_session::BBSession,
    course_code: String,
    semester: String,
    alias: String,
    out_dir: PathBuf,
    files_dir: PathBuf,
    temp_dir: PathBuf,
    tree_dir: PathBuf,
    id: String,
}

impl BBCourse {
    pub fn new(
        session: blackboard_session::BBSession, // Should be reference to session owned by course manager
        course_code: &str,
        semester: &str,
        alias: &str,
        out_dir: &Path,
        temp_dir: &Path,
        id: &str
    ) -> BBCourse {
        let files_dir = out_dir.join("downloaded_attachments");
        let tree_dir = out_dir.join("content_tree");
        std::fs::create_dir_all(&out_dir).expect("Error creating base folder");
        std::fs::create_dir_all(&files_dir).expect("Error creating files folder"); 
        std::fs::create_dir_all(&temp_dir).expect("Error creating temp folder");
        std::fs::create_dir_all(&tree_dir).expect("Error creating tree folder");
        BBCourse {
            session,
            course_code: course_code.to_string(),
            semester: semester.to_string(),
            alias: alias.to_string(),
            out_dir: out_dir.to_path_buf(),
            files_dir,
            temp_dir: temp_dir.to_path_buf(),
            tree_dir,
            // announcements_dir, ...
            id: id.to_string(),
        }
    }
    
    fn download_course_contents_json(&self, course_id: &str, query_parameters: &[&str], out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        let mut url = format!("https://{}/learn/api/public/v1/courses/{}/contents",
            self.session.domain,
            course_id);
        
        if !query_parameters.is_empty() {
            url.extend(format!("?{}", query_parameters.join("&")).chars());
        }

        self.session.download_file(&url, out_path)
    }

    fn download_course_root_contents_json(&self, course_id: &str, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        self.download_course_contents_json(course_id, &[blackboard_content::BBContent::DEFAULT_FIELDS], out_path)
    }
    
    fn download_course_announcements_json(&self, course_id: &str, query_parameters: &[&str], out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        // let fields = "id,title,contentHandler"; Alle egentlig interessante
        
        let mut url = format!("https://{}/learn/api/public/v1/courses/{}/announcements",
            self.session.domain,
            course_id);

        if !query_parameters.is_empty() {
            url.extend(format!("?{}", query_parameters.join("&")).chars());
        }

        self.session.download_file(&url, out_path)
    }

    fn get_course_root_content(&self) -> Result<Vec<BBContent>, Box<dyn std::error::Error>> {
        let root_content_json_filename = "root_content.json";
        let root_content_json_path = self.temp_dir.join(&root_content_json_filename);
        self.download_course_root_contents_json(&self.id, &root_content_json_path)?;
        BBContent::vec_from_json_results(&root_content_json_path, &self)
    }

    pub fn download_course_content_tree(
        &self, 
        content_predicate: Option<&dyn Fn(&BBContent) -> bool>, 
        attachment_predicate: Option<&dyn Fn(&BBAttachment) -> bool>,
        unzip: bool, 
        overwrite: bool
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut total_download_size = 0.0;
        // std::fs::create_dir_all(&self.tree_dir).expect("Error creating tree dir"); //Hvorfor klagde ikke denne når jeg hadde "?"?
        for content in self.get_course_root_content()? {
            total_download_size += content.download_children(content_predicate, attachment_predicate, &self.tree_dir, unzip, overwrite)?;
        }
        Ok(total_download_size)
    }

    //Announcements
    fn get_course_announcements(&self, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<BBAnnouncement>, Box<dyn std::error::Error>> {
        let announcements_json_filename = "announcements.json";
        let announcements_json_path = self.temp_dir.join(&announcements_json_filename);

        let mut query_parameters = Vec::new();
        if let Some(limit) = limit {
            query_parameters.push(format!("limit={}", limit));
        }
        if let Some(offset) = offset {
            query_parameters.push(format!("offset={}", offset));
        }

        let borrowed_query_parameters: Vec<&str> = query_parameters.iter().map(|s| s.as_str()).collect();

        self.download_course_announcements_json(&self.id, &borrowed_query_parameters[..], &announcements_json_path)?;
        BBAnnouncement::vec_from_json_results(&announcements_json_path)
    }
    
    pub fn view_course_announcements(&self, limit: Option<usize>, offset: Option<usize>) -> Result<(), Box<dyn std::error::Error>> {
        let announcements = self.get_course_announcements(limit, offset)?;
        if announcements.is_empty() {
            println!("No announcements found.")
        } else {
            for announcement in self.get_course_announcements(limit, offset)? {
                announcement.view();
            }
        }
        Ok(())
    }
    
    // pub fn download_course_assessment_questions_json(...)
}

impl super::Course for BBCourse {
    fn get_alias(&self) -> &str {
        &self.alias
    }

    fn set_alias(&mut self, new_alias: &str) {
        self.alias = String::from(new_alias);
    }

    fn get_course_code(&self) -> &str {
        &self.course_code
    }

    fn get_semester(&self) -> &str {
        &self.semester
    }
}


impl Drop for BBCourse {
    fn drop(&mut self) {
        if self.temp_dir.exists() {
            std::fs::remove_dir_all(&self.temp_dir).expect("Error deleting temp_dir");
        }
    }
}

impl std::convert::From<&BBCourse> for json::JsonValue {
    fn from(course: &BBCourse) -> json::JsonValue {
        json::object!{
            session: json::JsonValue::from(&course.session),
            course_code: course.course_code.clone(),
            semester: course.semester.clone(),
            alias: course.alias.clone(),
            out_dir: course.out_dir.as_os_str().to_str().unwrap(),
            temp_dir: course.temp_dir.as_os_str().to_str().unwrap(),
            id: course.id.clone(),
        }
    }
}

impl std::convert::From<json::JsonValue> for BBCourse {
    fn from(course: json::JsonValue) -> BBCourse {
        BBCourse::new(
            course["session"].clone().into(),
            &course["course_code"].to_string(),
            &course["semester"].to_string(),
            &course["alias"].to_string(),
            Path::new(&course["out_dir"].to_string()),
            Path::new(&course["temp_dir"].to_string()),
            &course["id"].to_string(),
        )
    }
}