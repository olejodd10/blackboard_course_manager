pub struct BBGradebookColumn {
    pub id: String,
    pub name: String,
    pub content_id: String,
    pub due: String, 
}

impl BBGradebookColumn {
    pub fn vec_from_json_results(json: Vec<u8>) -> Result<Vec<BBGradebookColumn>, Box<dyn std::error::Error>> {
        let json_string = std::string::String::from_utf8(json)?;
        let parsed_json = json::parse(&json_string)?;

        Ok(parsed_json["results"].members().map(|member| {
            BBGradebookColumn {
                id: member["id"].to_string(),
                name: member["name"].to_string(),
                content_id: member["contentId"].to_string(),
                due: member["grading"]["due"].to_string(), 
            }
        }).collect())
    }

    pub fn view(&self) {
        println!("\"{}\": due {}", self.name, self.due);
    }
}