const ALLOWED_NAME_LENGTH: usize = 5; // File creation causes error if absolute file path exceeds ~255-260 characters

const BANNED_FILENAME_CHARS: [char; 6] = [
    'æ',
    'ø',
    'å',
    'Æ',
    'Ø',
    'Å',
];

const BANNED_DIR_NAME_CHARS: [char; 13] = [
    '.',
    '/',
    '\\',
    ',',
    ' ',
    ':',
    '*',
    '\"',
    '?',
    '\'',
    '|',
    '<',
    '>',
];

pub fn valid_dir_name(s: &str) -> String {
    let mut s = s.replace(&BANNED_DIR_NAME_CHARS[..], "_");
    s.truncate(ALLOWED_NAME_LENGTH);
    s
}

pub fn valid_filename(s: &str) -> String {
    let mut splits_rev = s.split('.').rev();
    let extension = splits_rev.next().unwrap();
    let mut filename = splits_rev.rev().collect::<Vec<&str>>().join("_").replace(&BANNED_FILENAME_CHARS[..], "x");
    filename.truncate(ALLOWED_NAME_LENGTH-extension.len()-1); // Leave space for ".pdf", for example
    filename.push('.');
    filename.push_str(&extension);
    filename
}

// fjerne unødvendige deler av filnavnet. For eksempel er fagkoden alltid overflødig
//pub fn simplify_name(&str) -> String {

// }