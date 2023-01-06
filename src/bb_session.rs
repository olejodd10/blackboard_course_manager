
use std::io::Write;
use std::path::{Path, PathBuf};
use curl::easy::{Easy, Handler, WriteError};

const PATH_LENGTH_WARNING_LIMIT: usize = 230;

// https://docs.rs/curl/0.4.38/curl/easy/trait.Handler.html
struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

#[derive(Debug, Clone)]
pub struct BBSession {
    pub domain: String,
    pub cookie_jar_path: PathBuf,
}

impl BBSession {
    pub fn new(domain: &str, cookie_jar_path: &Path) -> Result<BBSession, Box<dyn std::error::Error>> {
        if !cookie_jar_path.exists() {
            println!("Please export cookies from domain \"{}\" to following path: \n{}\nPress enter when done.", domain, cookie_jar_path.to_str().unwrap());
            std::io::stdin().read_line(&mut String::new()).unwrap();
        }
        let bb_session = BBSession {
            domain: domain.to_string(),
            cookie_jar_path: cookie_jar_path.to_path_buf(),
        };
        if bb_session.test_connection() {
            Ok(bb_session)
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, 
                format!("Session connection test failed. Are cookies at {} up to date?", bb_session.cookie_jar_path.to_str().unwrap())
            )))
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

    pub fn download_file(&self, url: &str, out_path: &Path) -> Result<f64, Box<dyn std::error::Error>> {

        if let Ok(absolute_path) = out_path.canonicalize() {
            if absolute_path.to_str().unwrap().len() > PATH_LENGTH_WARNING_LIMIT {
                eprintln!("WARNING: Path length exceeds {} characters, and might approach system limit.", PATH_LENGTH_WARNING_LIMIT);
            }
        } 
        
        let mut out_file = std::fs::File::create(out_path).expect("Error creating out file");

        let mut easy = Easy::new();
        easy.url(url)?;
        easy.write_function(move |data| { 
            out_file.write_all(data).expect("Error writing data");
            Ok(data.len())
        })?;

        easy.cookie_file(&self.cookie_jar_path).unwrap();

        easy.follow_location(true)?; //Viktig fordi BB redirecter (302)
        easy.fail_on_error(true)?; //Viktig for å faile på 401
        
        easy.perform()?;
        
        Ok(easy.download_size()?)
    }

    pub fn download_bytes(&self, url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

        let mut buf = Vec::new();
        let mut easy = Easy::new();

        easy.url(url)?;

        easy.cookie_file(&self.cookie_jar_path).unwrap();

        easy.follow_location(true)?; //Viktig fordi BB redirecter (302)
        easy.fail_on_error(true)?; //Viktig for å faile på 401

        { // Scope to make transfer drop borrow of buf
            let mut transfer = easy.transfer();
            transfer.write_function(|data| { 
                buf.extend_from_slice(data);
                Ok(data.len())
            })?;
            transfer.perform()?;
        }
        
        Ok(buf)
    }

    pub fn download_courses_json(&self, query_parameters: &[&str]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut url = format!("https://{}/learn/api/public/v3/courses",
            self.domain);
        
        if !query_parameters.is_empty() {
            url.extend(format!("?{}", query_parameters.join("&")).chars());
        }

        self.download_bytes(&url)
    }
}


impl std::convert::From<&BBSession> for json::JsonValue {
    fn from(session: &BBSession) -> json::JsonValue {
        json::object!{
            domain: session.domain.clone(),
            cookie_jar_path: session.cookie_jar_path.as_os_str().to_str().unwrap(),
        }
    }
}

impl std::convert::From<json::JsonValue> for BBSession {
    fn from(course: json::JsonValue) -> BBSession {
        BBSession::new(
            &course["domain"].to_string(), 
            Path::new(&course["cookie_jar_path"].to_string())
        ).expect("Error creating new BBSession from JSON")
    }
}