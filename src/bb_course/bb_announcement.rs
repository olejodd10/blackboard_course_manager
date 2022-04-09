pub struct BBAnnouncement {
    pub id: String,
    pub title: String,
    pub body: String,
    pub creator: String, //OBS!! Er en id, ikke string
    pub created: String, // Endre til timestamp
    pub modified: String, // Endre til timestamp
}

impl BBAnnouncement {
    const VIEW_WIDTH: usize = 120;

    pub fn vec_from_json_results(json: Vec<u8>) -> Result<Vec<BBAnnouncement>, Box<dyn std::error::Error>> {
        let json_string = std::string::String::from_utf8(json)?;
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