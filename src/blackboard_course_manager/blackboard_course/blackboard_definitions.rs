// Alle 'a-lifetimene er veldig gode å ha, for de viser avhengigheten BBSession

// Mulige contenttyper:
// x-bb-file
// x-bb-document
// x-bb-assignment
// x-bb-folder
// bb-panopto-bc-mashup

use std::path::Path;

// https://docs.blackboard.com/learn/rest/advanced/contenthandler-datatypes
#[derive(Debug)]
pub enum BBContentHandler {
    
    // ContentHandlers with attachments
    XBBFile,
    XBBDocument,
    XBBAssignment,
    
    // ContentHandlers with children
    XBBFolder,

    // ContentHandlers with neither children nor attachments
    XBBForumlink, 
    XBBCourselink,
    XBBBLTILink,
    XBBExternallink,
    XBBASMTTestLink,

    BBPanoptoBCMashup, // BBPanoptoBCMashup has a body attribute with an iframe. The same media can be found by following the attached link.  

    XBBBlankpage,
    
    Unsupported,
    Undefined,
}

impl BBContentHandler {
    pub fn new(content_handler: &str) -> BBContentHandler {
        match content_handler {
            "resource/x-bb-file" => BBContentHandler::XBBFile,
            "resource/x-bb-document" => BBContentHandler::XBBDocument,
            "resource/x-bb-assignment" => BBContentHandler::XBBAssignment,
            "resource/x-bb-folder" => BBContentHandler::XBBFolder,
            "resource/x-bb-forumlink" => BBContentHandler::XBBForumlink,
            "resource/bb-panopto-bc-mashup" => BBContentHandler::BBPanoptoBCMashup,
            "resource/x-bb-asmt-test-link" => BBContentHandler::XBBASMTTestLink,
            "resource/x-bb-blankpage" => BBContentHandler::XBBBlankpage,
            "resource/x-bb-blti-link" |
            "resource/x-bb-externallink" |
            "resource/x-bb-courselink" => {
                eprintln!("Note: BlackBoard content handler \"{}\" is not yet supported.", content_handler);
                BBContentHandler::Unsupported
            },
            _ => {
                eprintln!("Note: Undefined BlackBoard content handler \"{}\".", content_handler);
                BBContentHandler::Undefined
            },
        }
    }
}

#[derive(Debug)]
pub struct BBContent {
    pub id: String,
    pub title: String,
    pub content_handler: BBContentHandler,
    pub links: Vec<String>, 
}

impl BBContent {
    const VIEW_WIDTH: usize = 50;

    pub fn vec_from_json_results(json_path: &Path) -> Result<Vec<BBContent>, Box<dyn std::error::Error>> {
        let json_string = std::fs::read_to_string(&json_path)?;
        let parsed_json = json::parse(&json_string)?;

        Ok(parsed_json["results"].members().map(|m1| {
            BBContent {
                id: m1["id"].to_string(),
                title: m1["title"].to_string(),
                content_handler: BBContentHandler::new(&m1["contentHandler"]["id"].to_string()),
                links: m1["links"].members().map(|m2| m2["href"].to_string()).collect(),
            }
        }).collect())
    }

    pub fn view(&self) {
        println!("{}\nTITLE: {}\nCONTENT HANDLER: {:?}\n",
            "*".repeat(BBContent::VIEW_WIDTH),
            self.title,
            self.content_handler
        );
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

    pub fn is_zip(&self) -> bool {
        self.mimetype == "application/zip"
    }

    pub fn view(&self) {
        println!("FILENAME: {}\nMIMETYPE: {}\n", self.filename, self.mimetype)
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
    const VIEW_WIDTH: usize = 50;

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

    pub fn view(&self) {
        println!("{}\nTITLE: {}\nCREATOR: {}\nCREATED: {}\nMODIFIED: {}\n{}\n{}\n",
            "*".repeat(BBAnnouncement::VIEW_WIDTH),
            self.title,
            self.creator,
            self.created,
            self.modified,
            "-".repeat(BBAnnouncement::VIEW_WIDTH),
            html2text::from_read(self.body.as_bytes(), BBAnnouncement::VIEW_WIDTH), 
        );
    }
}

// pub struct BBUser {

// }
