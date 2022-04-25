pub mod bb_attachment;
pub mod bb_content_classes;

use std::path::Path;
use std::io::Write;
use std::thread::JoinHandle;
use super::BBCourse;
use crate::utils::filename_utils::{valid_filename, valid_dir_name};
use crate::utils::time_utils::is_more_recent;
use bb_attachment::BBAttachment;

pub struct BBContent<'a, 'b> {
    pub course: &'a BBCourse<'b>,
    pub id: String,
    pub title: String,
    pub modified: String,
    pub content_handler: String,
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
                content_handler: m1["contentHandler"]["id"].to_string(),
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
        out_path: &Path, 
        overwrite: bool,
        threads: &mut Vec<JoinHandle<f64>>
    ) -> Result<(), Box<dyn std::error::Error>> {
        if bb_content_classes::ATTACHABLE.contains(&self.content_handler.as_str()) {
            let maybe_updated = is_more_recent(&self.modified, &self.course.last_tree_download);
            if overwrite || maybe_updated.is_none() || maybe_updated.is_some() && maybe_updated.unwrap() {
                let attachments_path = out_path.join(&valid_dir_name(&self.title));
                std::fs::create_dir_all(&attachments_path).expect("Error creating attachment files dir"); 
                self.download_attachments(&attachments_path, threads)
            } else {
                Ok(())
            }
        } else if self.content_handler == bb_content_classes::FOLDER {
            // "modified" for folders don't reflect their content, so no need in checking it.
            let children_path = out_path.join(&valid_dir_name(&self.title));
            std::fs::create_dir_all(&children_path).expect("Error creating children dir"); 
            match self.get_children() {
                Ok(children) => {
                    for child in children {
                        child.download_children(&children_path, overwrite, threads)?;
                    }
                },
                Err(err) => {
                    //TODO: Graceful handling only for HTTP 403
                    eprintln!("Error downloading children for \"{}\": {}", self.title, err);
                }
            }
            Ok(())
        } else {
            let maybe_updated = is_more_recent(&self.modified, &self.course.last_tree_download);
            if !self.links.is_empty() && (overwrite || maybe_updated.is_none() || maybe_updated.is_some() && maybe_updated.unwrap()) {
                // eprintln!("No branching action defined for {} with content handler {:?}; saving links file instead", self.title, self.content_handler);
                let links_file_path = out_path.join(&format!("{}_links.txt", &valid_filename(&self.title)));
                self.create_links_file(&links_file_path)?;
                Ok(())
            } else {
                Ok(())
            }
        }
    }
    
    // Course content tree
    fn download_attachments(
        &self, 
        out_path: &Path,
        threads: &mut Vec<JoinHandle<f64>>
    ) -> Result<(), Box<dyn std::error::Error>> {
        let content_attachments = self.get_attachments()?;
        for attachment in content_attachments {
            let file_path = out_path.join(&valid_filename(&attachment.filename));
            attachment.download(&file_path, threads)?;
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
