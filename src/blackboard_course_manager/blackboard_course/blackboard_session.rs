pub mod filename_utils;
pub mod input_utils;

use filename_utils::cookie_filename;
use input_utils::stdin_trimmed_line;

use std::path::{Path, PathBuf};
use std::io::{Read, Write, BufWriter, BufRead};

use curl::easy::{Easy2, Easy, Handler, WriteError};

// https://docs.rs/curl/0.4.38/curl/easy/trait.Handler.html
struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

const SYNC_CHANNEL_BUFFER_SIZE: usize = 1000000;

#[derive(Debug)]
pub struct BBSession {
    pub domain: String,
    pub cookie_jar_dir: PathBuf,
    pub cookie_jar_path: PathBuf,
}

impl BBSession {
    pub fn new(domain: &str, cookie_jar_dir: &Path) -> Result<BBSession, Box<dyn std::error::Error>> {
        std::fs::create_dir_all(cookie_jar_dir).expect("Error creating BBSession cookie_jar_dir");
        let cookie_jar_path = cookie_jar_dir.join(cookie_filename(domain));
        let bb_session = BBSession {
            domain: domain.to_string(),
            cookie_jar_dir: cookie_jar_dir.to_path_buf(),
            cookie_jar_path,
        };
        bb_session.connect()?;
        Ok(bb_session)
    }

    pub fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.test_connection() {
            Ok(())
        } else {
            eprintln!("No existing connection found.");
            std::fs::create_dir_all(&self.cookie_jar_dir)?;
            let cookie_jar_path = self.cookie_jar_dir.join(cookie_filename(&self.domain));
            if !cookie_jar_path.exists() {
                panic!("Please export cookies from domain \"{}\" to following path: \n{}", self.domain, cookie_jar_path.to_str().unwrap());
            }
            let bb_session = BBSession {
                domain: self.domain.to_string(),
                cookie_jar_dir: self.cookie_jar_dir.to_path_buf(),
                cookie_jar_path,
            };
            
            if bb_session.test_connection() {
                Ok(())
            } else {
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Initiated session connection test failed")))
            }
        }
    }

    pub fn test_connection(&self) -> bool {
        if self.cookie_jar_path.exists() {
            let mut easy = Easy::new();
            easy.url(&format!("https://{}/learn/api/public/v1/courses", self.domain)).unwrap();
            easy.cookie_file(&self.cookie_jar_path).unwrap();
            easy.fail_on_error(true).unwrap();
            easy.perform().is_ok()
        } else {
            false
        }
    }
}


impl std::convert::From<&BBSession> for json::JsonValue {
    fn from(session: &BBSession) -> json::JsonValue {
        json::object!{
            domain: session.domain.clone(),
            cookie_jar_dir: session.cookie_jar_dir.as_os_str().to_str().unwrap(),
        }
    }
}

impl std::convert::From<json::JsonValue> for BBSession {
    fn from(course: json::JsonValue) -> BBSession {
        BBSession::new(
            &course["domain"].to_string(), 
            Path::new(&course["cookie_jar_dir"].to_string())
        ).expect("Error creating new BBSession from JSON")
    }
}