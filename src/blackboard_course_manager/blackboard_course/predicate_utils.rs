use super::blackboard_definitions::{BBAttachment, BBContent};

fn is_substring(substring: &str, string: &str) -> bool {
    string.find(substring).is_some()
}

pub fn mimetype_substring(attachment: &BBAttachment, substring: &str) -> bool {
    is_substring(substring, &attachment.mimetype)
}

pub fn small_file_mimetype(attachment: &BBAttachment) -> bool {
    !is_substring("video", &attachment.mimetype)
}

pub fn filename_substring(
    attachment: &BBAttachment, 
    substring: &str
) -> bool {
    is_substring(substring, &attachment.filename)
}

pub fn title_substring(
    content: &BBContent, 
    substring: &str) -> bool {
    is_substring(substring, &content.title)
}


