// Alle 'a-lifetimene er veldig gode å ha, for de viser avhengigheten BBSession

// Mulige contenttyper:
// x-bb-file
// x-bb-document
// x-bb-assignment
// x-bb-folder
// x-bb-assignment
// bb-panopto-bc-mashup

use super::BBCourse;

#[derive(Debug)]
pub struct BBContent<'a> {
    pub session: &'a BBSession,
    pub course: &'a 
    pub attachments: Vec<BBAttachment>,
    pub id: String,
    pub title: String,
}

impl<'a> BBContent<'a> {
    pub fn fetch_attachments(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let session = course.session;

        let json_filename = format!("{}_attachments.json", self.id);
        let json_path = course.out_dir.join(&json_filename);

        session.download_content_attachments_json(&course.id, &self.id, &json_path)?;

        let json_string = std::fs::read_to_string(&json_path)?;
        let parsed_json = json::parse(&json_string)?;

        self.attachments = parsed_json["results"].members().map(|member| {
            BBAttachment {
                id: member["id"].to_string(),
                filename: member["fileName"].to_string(),
                mimetype: member["mimeType"].to_string(),           
            }
        }).collect();

        Ok(())
    }
}


#[derive(Clone, Debug)]
pub struct BBAttachment {
    pub id: String,
    pub filename: String,
    pub mimetype: String,
}

impl BBAttachment {
    fn download(&self, course: &BBCourse, content_id: &str) -> Result<f64, Box<dyn std::error::Error>> {
        let session = course.session;
        let course_id = &course.id;
        let out_dir = &course.out_dir;
        session.download_content_attachment(course_id, content_id, &self.id, out_dir)
    }
}


// pub struct BBAnnouncement {
//     ...
// }
