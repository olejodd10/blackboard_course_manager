use crate::bb_session::BBSession;

pub struct BBUser {
    _user_name: String,
    name: String,
    _id: String,
}

impl BBUser {

    fn get_by_id(session: &BBSession, id: &str) -> Result<BBUser, Box<dyn std::error::Error>> {
        let url = format!("https://{}/learn/api/public/v1/users/{}", session.domain, id);

        let bytes = session.download_bytes(&url)?;

        let json_string = std::string::String::from_utf8(bytes)?;
        let parsed_json = json::parse(&json_string)?;

        Ok(BBUser {
            _user_name: parsed_json["userName"].to_string(),
            name: format!("{} {}", parsed_json["name"]["given"], parsed_json["name"]["family"]),
            _id: parsed_json["id"].to_string(),
        })
    }

    pub fn name_by_id(session: &BBSession, id: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(BBUser::get_by_id(session, id)?.name)
    }
}
    