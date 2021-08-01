pub fn mimetype_match(
    attachment: &super::blackboard_definitions::BBAttachment, 
    mimetype: &str
) -> bool {
    attachment.mimetype == mimetype
}

pub fn filename_substring(
    attachment: &super::blackboard_definitions::BBAttachment, 
    filename_substring: &str
) -> bool {
    attachment.filename.find(filename_substring).is_some()
}

pub fn title_substring(
    content: &super::blackboard_definitions::BBContent, 
    title_substring: &str) -> bool {
    content.title.find(title_substring).is_some()
}