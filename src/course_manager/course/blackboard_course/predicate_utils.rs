use super::blackboard_definitions::{BBAttachment, BBContent};

pub fn mimetype_match(attachment: &BBAttachment, mimetype: &str) -> bool {
    attachment.mimetype == mimetype
}

pub fn small_file_mimetype(attachment: &BBAttachment) -> bool {
    attachment.mimetype.find("video").is_none()
}

pub fn filename_substring(
    attachment: &BBAttachment, 
    filename_substring: &str
) -> bool {
    attachment.filename.find(filename_substring).is_some()
}

pub fn title_substring(
    content: &BBContent, 
    title_substring: &str) -> bool {
    content.title.find(title_substring).is_some()
}