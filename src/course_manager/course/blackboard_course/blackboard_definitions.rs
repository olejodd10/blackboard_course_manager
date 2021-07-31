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
    pub course: &'a BBCourse<'a>,
    pub id: String,
    pub title: String,
}

impl<'a> BBContent<'a> {
    pub fn fetch_attachments(&self) -> Result<Vec<BBAttachment>, Box<dyn std::error::Error>> {
        let course = self.course;
        let session = course.session;

        let json_filename = format!("{}_attachments.json", self.id);
        let json_path = course.out_dir.join(&json_filename);

        session.download_content_attachments_json(&course.id, &self.id, &json_path)?;

        let json_string = std::fs::read_to_string(&json_path)?;
        let parsed_json = json::parse(&json_string)?;

        let content_attachments = parsed_json["results"].members().map(|member| {
            BBAttachment {
                content: &self,
                id: member["id"].to_string(),
                filename: member["fileName"].to_string(),
                mimetype: member["mimeType"].to_string(),           
            }
        }).collect();

        Ok(content_attachments)
    }
}


#[derive(Clone, Debug)]
pub struct BBAttachment<'a> {
    pub content: &'a BBContent<'a>,
    pub id: String,
    pub filename: String,
    pub mimetype: String,
}

impl<'a> BBAttachment<'a> {
    fn download(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let course = self.content.course;
        let session = course.session;
        let course_id = &course.id;
        let content_id = &self.content.id;
        let out_dir = &course.out_dir;
        session.download_content_attachment(course_id, content_id, &self.id, out_dir)
    }
}


// pub struct BBAnnouncement {
//     ...
// }
