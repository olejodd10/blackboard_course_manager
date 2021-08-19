use std::path::{PathBuf, Path};
use std::io::Write;

pub mod blackboard_session;
pub mod blackboard_definitions;
pub mod predicate_utils;
use blackboard_definitions::{BBAttachment, BBContent, BBAnnouncement, BBContentHandler};
use blackboard_session::filename_utils::{valid_dir_name, valid_filename};

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
        session: blackboard_session::BBSession,
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

    // Everything it takes to create the course content tree
    fn get_course_root_content(&self) -> Result<Vec<BBContent>, Box<dyn std::error::Error>> {
        let root_content_json_filename = "root_content.json";
        let root_content_json_path = self.temp_dir.join(&root_content_json_filename);
        self.session.download_course_root_contents_json(&self.id, &root_content_json_path)?;
        BBContent::vec_from_json_results(&root_content_json_path)
    }

    fn get_content_children(&self, content: &BBContent) -> Result<Vec<BBContent>, Box<dyn std::error::Error>> {
        let content_children_json_filename = format!("{}_children.json", valid_dir_name(&content.title));
        let content_children_json_path = self.temp_dir.join(&content_children_json_filename);
        self.session.download_content_children_json(&self.id, &content.id, &[blackboard_session::DEFAULT_FIELDS], &content_children_json_path)?;
        BBContent::vec_from_json_results(&content_children_json_path)
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
            total_download_size += self.download_children(&content, content_predicate, attachment_predicate, &self.tree_dir, unzip, overwrite)?;
        }
        eprintln!("Content tree download completed. Total download size: {} bytes.", total_download_size);
        Ok(total_download_size)
    }

    fn download_children(&self, 
        content: &BBContent, 
        content_predicate: Option<&dyn Fn(&BBContent) -> bool>, 
        attachment_predicate: Option<&dyn Fn(&BBAttachment) -> bool>,
        out_path: &Path, 
        unzip: bool, 
        overwrite: bool
    ) -> Result<f64, Box<dyn std::error::Error>> {
        if let Some(content_predicate) = content_predicate {
            if !content_predicate(content) {
                return Ok(0.0);
            }
        }
        match &content.content_handler {
            handler if blackboard_definitions::ATTACHABLE_CONTENT_HANDLERS.contains(handler) => {
                let attachments_path = out_path.join(&valid_dir_name(&content.title));
                std::fs::create_dir_all(&attachments_path).expect("Error creating attachment files dir"); 
                self.download_content_attachments(content, attachment_predicate, &attachments_path, unzip, overwrite) 
            },
            BBContentHandler::XBBFolder => {
                let children_path = out_path.join(&valid_dir_name(&content.title));
                let mut total_download_size = 0.0;
                std::fs::create_dir_all(&children_path).expect("Error creating children dir"); 
                for child in self.get_content_children(content)? {
                    total_download_size += self.download_children(&child, content_predicate, attachment_predicate, &children_path, unzip, overwrite)?;
                }
                Ok(total_download_size)
            },
            handler => {
                eprintln!("No branching action defined for {} with content handler {:?}; saving links file instead", content.title, handler);
                if !content.links.is_empty() {
                    let links_file_path = out_path.join(&format!("{}_links.txt", &valid_filename(&content.title)));
                    self.create_links_file(content, &links_file_path)
                } else {
                    Ok(0.0)
                }
            },
        }
    }

    fn create_links_file(&self, content: &BBContent, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        let mut links_file = std::fs::File::create(out_path).expect("Error creating links file");
        for link in &content.links {
            writeln!(links_file, "https://{}{}", self.session.domain, link).unwrap();
        }
        Ok(links_file.metadata()?.len() as f64)
    }

    //Announcements
    pub fn get_course_announcements(&self, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<BBAnnouncement>, Box<dyn std::error::Error>> {
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

        self.session.download_course_announcements_json(&self.id, &borrowed_query_parameters[..], &announcements_json_path)?;
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

    // Course content, to get specific files (not tree)
    pub fn get_course_contents(&self) -> Result<Vec<BBContent>, Box<dyn std::error::Error>> {
        let json_filename = "contents.json";
        let json_path = self.temp_dir.join(&json_filename);
        self.session.download_course_contents_json(&self.id, &["recursive=true"], &json_path)?;
        BBContent::vec_from_json_results(&json_path)
    }

    fn get_attachable_course_contents(&self) -> Result<Vec<BBContent>, Box<dyn std::error::Error>> {
        Ok(self.get_course_contents()?.into_iter()
            .filter(|content| blackboard_definitions::ATTACHABLE_CONTENT_HANDLERS.contains(&content.content_handler))
            .collect())
    }

    fn get_viewable_course_contents(&self) -> Result<Vec<BBContent>, Box<dyn std::error::Error>> {
        Ok(self.get_course_contents()?.into_iter()
            .filter(|content| blackboard_definitions::VIEWABLE_CONTENT_HANDLERS.contains(&content.content_handler))
            .collect())
    }
 
    pub fn view_course_contents(&self, content_predicate: Option<&dyn Fn(&BBContent) -> bool>) -> Result<(), Box<dyn std::error::Error>> {
        let mut contents = self.get_viewable_course_contents()?.into_iter().peekable();
        if contents.peek().is_none() {
            println!("No contents found.")
        } else {
            if let Some(content_predicate) = content_predicate {
                let mut filtered_contents = contents.filter(|content| content_predicate(content)).peekable();
                if filtered_contents.peek().is_none() {
                    println!("No contents passed through filter.");
                } else {                    
                    for content in filtered_contents {
                        content.view();
                    }
                }
            } else {
                for content in contents {
                    content.view();
                }
            }
        }
        Ok(())
    }

    // TODO: Can extend this with content predicate
    pub fn view_course_attachments(&self, attachment_predicate: Option<&dyn Fn(&BBAttachment) -> bool>) -> Result<(), Box<dyn std::error::Error>> {
        let mut attachments = self.get_attachable_course_contents()?.into_iter()
            .map(|content| self.get_content_attachments(&content).expect("Error getting content attachments").into_iter())
            .flatten()
            .peekable();
        if attachments.peek().is_none() {
            println!("No attachments found.")
        } else {
            if let Some(attachment_predicate) = attachment_predicate {
                let mut filtered_attachments = attachments.filter(|attachment| attachment_predicate(attachment)).peekable();
                if filtered_attachments.peek().is_none() {
                    println!("No attachments passed through filter.");
                } else {
                    for attachment in filtered_attachments {
                        attachment.view();
                    }
                }
            } else {
                for attachment in attachments {
                    attachment.view();
                }
            }
        }
        Ok(())
    }

    // Attachments
    pub fn get_content_attachments(&self, content: &BBContent) -> Result<Vec<BBAttachment>, Box<dyn std::error::Error>> {
        let attachments_json_filename = format!("{}_attachments.json", content.id);
        let attachments_json_path = self.temp_dir.join(&attachments_json_filename);
        self.session.download_content_attachments_json(&self.id, &content.id, &attachments_json_path)?;
        BBAttachment::vec_from_json_results(&attachments_json_path)
    }

    // Download attachments satisfying predicate, for specified content instance
    pub fn download_content_attachments(
        &self, 
        content: &BBContent, 
        attachment_predicate: Option<&dyn Fn(&BBAttachment) -> bool>,
        out_path: &Path,
        unzip: bool,
        overwrite: bool
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let content_attachments = self.get_content_attachments(content)?;
        let mut total_download_size = 0.0;
        if let Some(attachment_predicate) = attachment_predicate {
            for attachment in content_attachments.into_iter().filter(|attachment| attachment_predicate(attachment)) {
                let unzip = unzip && attachment.is_zip(); // Only unzip if unzip flag set, and file is zipped
                total_download_size += self.session.download_content_attachment(&self.id, &content.id, &attachment.id, &out_path.join(&valid_filename(&attachment.filename)), unzip, overwrite)?;
            }
        } else {
            for attachment in content_attachments {
                let unzip = unzip && attachment.is_zip(); // Only unzip if unzip flag set, and file is zipped
                total_download_size += self.session.download_content_attachment(&self.id, &content.id, &attachment.id, &out_path.join(&valid_filename(&attachment.filename)), unzip, overwrite)?;
            }
        }
        Ok(total_download_size)
    }

    // Download all attachments in course meeting predicates
    pub fn download_course_content_attachments(
        &self, 
        content_predicate: Option<&dyn Fn(&BBContent) -> bool>, 
        attachment_predicate: Option<&dyn Fn(&BBAttachment) -> bool>,
        unzip: bool,
        overwrite: bool
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let attachable_course_contents = self.get_attachable_course_contents()?;
        let mut total_download_size = 0.0;
        if let Some(content_predicate) = content_predicate {
            for content in attachable_course_contents.into_iter().filter(|content| content_predicate(content)) {
                total_download_size += self.download_content_attachments(&content, attachment_predicate, &self.files_dir, unzip, overwrite)?;
            }
        } else {
            for content in attachable_course_contents {
                total_download_size += self.download_content_attachments(&content, attachment_predicate, &self.files_dir, unzip, overwrite)?;
            }
        }
        Ok(total_download_size)
    }

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