pub mod bb_attachment;
pub mod time_utils;

use std::path::Path;
use std::io::Write;
use std::thread::JoinHandle;
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

pub struct BBContent<'a, 'b> {
    pub course: &'a BBCourse<'b>,
    pub id: String,
    pub title: String,
    pub modified: String,
    pub content_handler: BBContentHandler,
    pub links: Vec<String>, 
}

impl<'a, 'b> BBContent<'a, 'b> {
    pub const DEFAULT_FIELDS: &'static str = "fields=id,title,modified,contentHandler,links"; // Looks like all contentHandlers have these fields (not attachments, though).

    pub fn vec_from_json_results(json: Vec<u8>, course: &'a BBCourse<'b>) -> Result<Vec<BBContent<'a, 'b>>, Box<dyn std::error::Error>> {
        let json_string = std::string::String::from_utf8(json)?;
        let parsed_json = json::parse(&json_string)?;

        Ok(parsed_json["results"].members().map(|m1| {
            BBContent {
                course,
                id: m1["id"].to_string(),
                title: m1["title"].to_string(),
                modified: m1["modified"].to_string(),
                content_handler: BBContentHandler::new(&m1["contentHandler"]["id"].to_string()),
                links: m1["links"].members().map(|m2| m2["href"].to_string()).collect(),
            }
        }).collect())
    }

    fn get_children(&self) -> Result<Vec<BBContent>, Box<dyn std::error::Error>> {
        let json = self.download_children_json(&[BBContent::DEFAULT_FIELDS])?;
        BBContent::vec_from_json_results(json, self.course)
    }

    fn get_attachments(&self) -> Result<Vec<BBAttachment>, Box<dyn std::error::Error>> {
        let json = self.download_attachments_json()?;
        BBAttachment::vec_from_json_results(json, self)
    }

    fn download_attachments_json(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        
        let url = format!("https://{}/learn/api/public/v1/courses/{}/contents/{}/attachments",
            self.course.manager.session.domain,
            self.course.id,
            self.id);
    
        self.course.manager.session.download_bytes(&url)
    }
    
    fn download_children_json(&self, query_parameters: &[&str]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut url = format!("https://{}/learn/api/public/v1/courses/{}/contents/{}/children",
            self.course.manager.session.domain,
            self.course.id,
            self.id);
    
        if !query_parameters.is_empty() {
            url.extend(format!("?{}", query_parameters.join("&")).chars());
        }

        self.course.manager.session.download_bytes(&url)
    }

    pub fn download_children(&self, 
        content_predicate: Option<&dyn Fn(&BBContent) -> bool>, 
        attachment_predicate: Option<&dyn Fn(&BBAttachment) -> bool>,
        out_path: &Path, 
        overwrite: bool,
        threads: &mut Vec<JoinHandle<f64>>
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(content_predicate) = content_predicate {
            if !content_predicate(self) {
                return Ok(());
            }
        }
        match &self.content_handler {
            handler if ATTACHABLE_CONTENT_HANDLERS.contains(handler) => {
                let maybe_updated = time_utils::is_more_recent(&self.modified, &self.course.last_tree_download);
                if overwrite || maybe_updated.is_none() || maybe_updated.is_some() && maybe_updated.unwrap() {
                    let attachments_path = out_path.join(&valid_dir_name(&self.title));
                    std::fs::create_dir_all(&attachments_path).expect("Error creating attachment files dir"); 
                    self.download_attachments(attachment_predicate, &attachments_path, threads)
                } else {
                    Ok(())
                }
            },
            BBContentHandler::XBBFolder => {
                // "modified" for folders don't reflect their content, so no need in checking it.
                let children_path = out_path.join(&valid_dir_name(&self.title));
                std::fs::create_dir_all(&children_path).expect("Error creating children dir"); 
                match self.get_children() {
                    Ok(children) => {
                        for child in children {
                            child.download_children(content_predicate, attachment_predicate, &children_path, overwrite, threads)?;
                        }
                    },
                    Err(err) => {
                        //TODO: Graceful handling only for HTTP 403
                        eprintln!("Error downloading children for \"{}\": {}", self.title, err);
                    }
                }
                Ok(())
            },
            handler => {
                let maybe_updated = time_utils::is_more_recent(&self.modified, &self.course.last_tree_download);
                if !self.links.is_empty() && (overwrite || maybe_updated.is_none() || maybe_updated.is_some() && maybe_updated.unwrap()) {
                    eprintln!("No branching action defined for {} with content handler {:?}; saving links file instead", self.title, handler);
                    let links_file_path = out_path.join(&format!("{}_links.txt", &valid_filename(&self.title)));
                    self.create_links_file(&links_file_path)?;
                    Ok(())
                } else {
                    Ok(())
                }
            },
        }
    }
    
    // Course content tree
    fn download_attachments(
        &self, 
        attachment_predicate: Option<&dyn Fn(&BBAttachment) -> bool>,
        out_path: &Path,
        threads: &mut Vec<JoinHandle<f64>>
    ) -> Result<(), Box<dyn std::error::Error>> {
        let content_attachments = self.get_attachments()?;
        if let Some(attachment_predicate) = attachment_predicate {
            for attachment in content_attachments.into_iter().filter(|attachment| attachment_predicate(attachment)) {
                let file_path = out_path.join(&valid_filename(&attachment.filename));
                attachment.download(&file_path, threads)?;
            }
        } else {
            for attachment in content_attachments {
                let file_path = out_path.join(&valid_filename(&attachment.filename));
                attachment.download(&file_path, threads)?;
            }
        }
        Ok(())
    }

    fn create_links_file(&self, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {
        let mut links_file = std::fs::File::create(out_path).expect("Error creating links file");
        for link in &self.links {
            writeln!(links_file, "https://{}{}", self.course.manager.session.domain, link).unwrap();
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
