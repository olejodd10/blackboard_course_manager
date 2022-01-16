use super::bb_content::BBContent;
use super::bb_content::bb_attachment::BBAttachment;

pub fn mimetype_contains(attachment: &BBAttachment, substring: &str) -> bool {
    attachment.mimetype.contains(substring)
}

pub fn small_file_mimetype(attachment: &BBAttachment) -> bool {
    !attachment.mimetype.contains("video")
}

pub fn filename_contains(
    attachment: &BBAttachment, 
    substring: &str
) -> bool {
    attachment.filename.contains(substring)
}

pub fn title_contains(
    content: &BBContent, 
    substring: &str) -> bool {
    content.title.contains(substring)
}


