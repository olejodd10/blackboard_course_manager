// Alle 'a-lifetimene er veldig gode å ha, for de viser avhengigheten BBSession

// Mulige contenttyper:
// x-bb-file
// x-bb-document
// x-bb-assignment
// x-bb-folder
// x-bb-assignment
// bb-panopto-bc-mashup
use std::path::Path;

use super::BBCourse;

#[derive(Debug)]
pub struct BBContent {
    pub attachments: Vec<BBAttachment>,
    pub id: String,
    pub title: String,
}

impl BBContent {
    pub fn vec_from_json_results(json_path: &Path) -> Result<Vec<BBContent>, Box<dyn std::error::Error>> {
        let json_string = std::fs::read_to_string(&json_path)?;
        let parsed_json = json::parse(&json_string)?;

        Ok(parsed_json["results"].members().map(|member| {
            BBContent {
                attachments: Vec::new(),
                id: member["id"].to_string(),
                title: member["title"].to_string(),
            }
        }).collect())
    }
}


#[derive(Clone, Debug)]
pub struct BBAttachment {
    pub id: String,
    pub filename: String,
    pub mimetype: String,
}

impl BBAttachment {
    pub fn vec_from_json_results(json_path: &Path) -> Result<Vec<BBAttachment>, Box<dyn std::error::Error>> {
        let json_string = std::fs::read_to_string(&json_path)?;
        let parsed_json = json::parse(&json_string)?;

        Ok(parsed_json["results"].members().map(|member| {
            BBAttachment {
                id: member["id"].to_string(),
                filename: member["fileName"].to_string(),
                mimetype: member["mimeType"].to_string(),           
            }
        }).collect())
    }
}


// pub struct BBAnnouncement {
//     ...
// }
