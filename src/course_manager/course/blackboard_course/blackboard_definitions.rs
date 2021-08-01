// Alle 'a-lifetimene er veldig gode å ha, for de viser avhengigheten BBSession

// Mulige contenttyper:
// x-bb-file
// x-bb-document
// x-bb-assignment
// x-bb-folder
// x-bb-assignment
// bb-panopto-bc-mashup

use std::path::Path;

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


pub struct BBAnnouncement {
    pub id: String,
    pub title: String,
    pub body: String,
    pub creator: String, //OBS!! Er en id, ikke string
    pub created: String, // Endre til timestamp
    pub modified: String, // Endre til timestamp
}

impl BBAnnouncement {
    pub fn vec_from_json_results(json_path: &Path) -> Result<Vec<BBAnnouncement>, Box<dyn std::error::Error>> {
        let json_string = std::fs::read_to_string(&json_path)?;
        let parsed_json = json::parse(&json_string)?;

        Ok(parsed_json["results"].members().map(|member| {
            BBAnnouncement {
                id: member["id"].to_string(),
                title: member["title"].to_string(),
                body: member["body"].to_string(),
                creator: member["creator"].to_string(), 
                created: member["created"].to_string(), 
                modified: member["modified"].to_string(),
            }
        }).collect())
    }
}

// pub struct BBUser {

// }
