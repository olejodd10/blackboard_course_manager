pub fn mimetype_and_filename_substring(
    attachment: &super::blackboard_definitions::BBAttachment, 
    mimetype: &str, 
    filename_substring: &str) -> bool {
    attachment.mimetype == mimetype && attachment.filename.find(filename_substring).is_some()
}