const ALLOWED_NAME_LENGTH: usize = 20; // File creation causes error if absolute file path exceeds ~255-260 characters

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

fn safe_truncate(s: &mut String, new_len: usize) {
    *s = s.chars().take(new_len).collect();
}

pub fn valid_dir_name(s: &str) -> String {
    let mut s = s.replace(&BANNED_DIR_NAME_CHARS[..], "_");
    safe_truncate(&mut s, ALLOWED_NAME_LENGTH);
    s
}

pub fn valid_filename(s: &str) -> String {
    let mut splits_rev = s.split('.').rev();
    let extension = splits_rev.next().unwrap();
    let mut filename = splits_rev.rev().collect::<Vec<&str>>().join("_").replace(&BANNED_FILENAME_CHARS[..], "x");
    // THIS LINE IS SUSPECTED DANGEROUS. "assertion failed: self.is_char_boundary(new_len)" and not that useful
    // filename.truncate(ALLOWED_NAME_LENGTH-extension.len()-1); // Leave space for ".pdf", for example
    filename.push('.');
    filename.push_str(&extension);
    filename
}

// fjerne unødvendige deler av filnavnet. For eksempel er fagkoden alltid overflødig
//pub fn simplify_name(&str) -> String {

// }