const BANNED_FILENAME_CHARS: [char; 14] = [
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
    '-',
];

pub fn valid_filename(s: &str) -> String {
    s.replace(&BANNED_FILENAME_CHARS[..], "_")
}