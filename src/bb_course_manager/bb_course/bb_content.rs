pub mod bb_attachment;

use std::path::Path;
use std::io::Write;
use super::{BBCourse, filename_utils::valid_filename, filename_utils::valid_dir_name};
use bb_attachment::BBAttachment;

// https://docs.blackboard.com/learn/rest/advanced/contenthandler-datatypes
#[derive(Debug, PartialEq)]
pub enum BBContentHandler {
    BBPanoptoBCMashup, // BBPanoptoBCMashup has a body attribute with an iframe. The same media can be found by following the attached link.  
    Null,
    XBBASMTTestLink,
    XBBAssignment,
    XBBBlankpage,
    XBBBLTILink,
    XBBCourselink,
    XBBDocument,
    XBBExternallink,
    XBBFile,
    XBBFolder,
    XBBForumlink, 
    XBBToollink,
    Undefined,
}

pub const ATTACHABLE_CONTENT_HANDLERS: [BBContentHandler; 3] = [
    BBContentHandler::XBBAssignment,
    BBContentHandler::XBBDocument,
    BBContentHandler::XBBFile,
];

pub const VIEWABLE_CONTENT_HANDLERS: [BBContentHandler; 13] = [
    BBContentHandler::BBPanoptoBCMashup,
    BBContentHandler::Null,
    BBContentHandler::XBBASMTTestLink,
    BBContentHandler::XBBAssignment,
    BBContentHandler::XBBBlankpage,
    BBContentHandler::XBBBLTILink,
    BBContentHandler::XBBCourselink,
    BBContentHandler::XBBDocument,
    BBContentHandler::XBBExternallink,
    BBContentHandler::XBBFile,
    BBContentHandler::XBBForumlink, 
    BBContentHandler::XBBToollink, 
    BBContentHandler::Undefined,
];


impl BBContentHandler {
    pub fn new(content_handler: &str) -> BBContentHandler {
        match content_handler {
            "resource/bb-panopto-bc-mashup" => BBContentHandler::BBPanoptoBCMashup,
            "null" => BBContentHandler::Null,
            "resource/x-bb-asmt-test-link" => BBContentHandler::XBBASMTTestLink,
            "resource/x-bb-assignment" => BBContentHandler::XBBAssignment,
            "resource/x-bb-blankpage" => BBContentHandler::XBBBlankpage,
            "resource/x-bb-document" => BBContentHandler::XBBDocument,
            "resource/x-bb-externallink" => BBContentHandler::XBBExternallink,
            "resource/x-bb-file" => BBContentHandler::XBBFile,
            "resource/x-bb-folder" => BBContentHandler::XBBFolder,
            "resource/x-bb-forumlink" => BBContentHandler::XBBForumlink,
            "resource/x-bb-toollink" => BBContentHandler::XBBToollink,
            _ => {
                eprintln!("Note: Undefined BlackBoard content handler \"{}\".", content_handler);
                BBContentHandler::Undefined
            },
        }
    }
}

pub struct BBContent<'a> {
    pub course: &'a BBCourse,
    pub id: String,
    pub title: String,
    pub content_handler: BBContentHandler,
    pub links: Vec<String>, 
}

impl<'a> BBContent<'a> {
    const VIEW_WIDTH: usize = 120;
    pub const DEFAULT_FIELDS: &'static str = "fields=id,title,contentHandler,links"; // Looks like all contentHandlers have these fields (not attachments, though).

    pub fn vec_from_json_results(json_path: &Path, course: &'a BBCourse) -> Result<Vec<BBContent<'a>>, Box<dyn std::error::Error>> {
        let json_string = std::fs::read_to_string(&json_path)?;
        let parsed_json = json::parse(&json_string)?;

        Ok(parsed_json["results"].members().map(|m1| {
            BBContent {
                course,
                id: m1["id"].to_string(),
                title: m1["title"].to_string(),
                content_handler: BBContentHandler::new(&m1["contentHandler"]["id"].to_string()),
                links: m1["links"].members().map(|m2| m2["href"].to_string()).collect(),
            }
        }).collect())
    }

    fn get_children(&self) -> Result<Vec<BBContent>, Box<dyn std::error::Error>> {
        let content_children_json_filename = format!("{}_children.json", valid_dir_name(&self.title));
        let content_children_json_path = self.course.temp_dir.join(&content_children_json_filename);
        self.download_children_json(&[BBContent::DEFAULT_FIELDS], &content_children_json_path)?;
        BBContent::vec_from_json_results(&content_children_json_path, self.course)
    }

    fn get_attachments(&self) -> Result<Vec<BBAttachment>, Box<dyn std::error::Error>> {
        let attachments_json_filename = format!("{}_attachments.json", self.id);
        let attachments_json_path = self.course.temp_dir.join(&attachments_json_filename);
        self.download_attachments_json(&attachments_json_path)?;
        BBAttachment::vec_from_json_results(&attachments_json_path, self)
    }

    fn download_attachments_json(&self, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        
        let url = format!("https://{}/learn/api/public/v1/courses/{}/contents/{}/attachments",
            self.course.session.domain,
            self.course.id,
            self.id);
    
        self.course.session.download_file(&url, out_path)
    }
    
    fn download_children_json(&self, query_parameters: &[&str], out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        let mut url = format!("https://{}/learn/api/public/v1/courses/{}/contents/{}/children",
            self.course.session.domain,
            self.course.id,
            self.id);
    
        if !query_parameters.is_empty() {
            url.extend(format!("?{}", query_parameters.join("&")).chars());
        }

        self.course.session.download_file(&url, out_path)
    }

    pub fn download_children(&self, 
        content_predicate: Option<&dyn Fn(&BBContent) -> bool>, 
        attachment_predicate: Option<&dyn Fn(&BBAttachment) -> bool>,
        out_path: &Path, 
        unzip: bool, 
        overwrite: bool
    ) -> Result<f64, Box<dyn std::error::Error>> {
        if let Some(content_predicate) = content_predicate {
            if !content_predicate(self) {
                return Ok(0.0);
            }
        }
        match &self.content_handler {
            handler if ATTACHABLE_CONTENT_HANDLERS.contains(handler) => {
                let attachments_path = out_path.join(&valid_dir_name(&self.title));
                std::fs::create_dir_all(&attachments_path).expect("Error creating attachment files dir"); 
                self.download_attachments(attachment_predicate, &attachments_path, unzip, overwrite) 
            },
            BBContentHandler::XBBFolder => {
                let children_path = out_path.join(&valid_dir_name(&self.title));
                let mut total_download_size = 0.0;
                std::fs::create_dir_all(&children_path).expect("Error creating children dir"); 
                match self.get_children() {
                    Ok(children) => {
                        for child in children {
                            total_download_size += child.download_children(content_predicate, attachment_predicate, &children_path, unzip, overwrite)?;
                        }
                    },
                    Err(err) => {
                        //TODO: Graceful handling only for HTTP 403
                        eprintln!("Error downloading children for \"{}\": {}", self.title, err);
                    }
                }
                Ok(total_download_size)
            },
            handler => {
                eprintln!("No branching action defined for {} with content handler {:?}; saving links file instead", self.title, handler);
                if !self.links.is_empty() {
                    let links_file_path = out_path.join(&format!("{}_links.txt", &valid_filename(&self.title)));
                    self.create_links_file(&links_file_path)
                } else {
                    Ok(0.0)
                }
            },
        }
    }
    
    // Course content tree
    fn download_attachments(
        &self, 
        attachment_predicate: Option<&dyn Fn(&BBAttachment) -> bool>,
        out_path: &Path,
        unzip: bool,
        overwrite: bool
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let content_attachments = self.get_attachments()?;
        let mut total_download_size = 0.0;
        if let Some(attachment_predicate) = attachment_predicate {
            for attachment in content_attachments.into_iter().filter(|attachment| attachment_predicate(attachment)) {
                let file_path = out_path.join(&valid_filename(&attachment.filename));
                total_download_size += attachment.download(&file_path, unzip, overwrite)?;
            }
        } else {
            for attachment in content_attachments {
                let file_path = out_path.join(&valid_filename(&attachment.filename));
                total_download_size += attachment.download(&file_path, unzip, overwrite)?;
            }
        }
        Ok(total_download_size)
    }

    fn create_links_file(&self, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        let mut links_file = std::fs::File::create(out_path).expect("Error creating links file");
        for link in &self.links {
            writeln!(links_file, "https://{}{}", self.course.session.domain, link).unwrap();
        }
        Ok(links_file.metadata()?.len() as f64)
    }

    pub fn view(&self) {
        println!("TITLE: {}\nCONTENT HANDLER: {:?}\n",
            self.title,
            self.content_handler
        );
    }
}

// pub struct BBUser {

// }
