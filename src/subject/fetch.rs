use std::path::{Path, PathBuf};
use std::io::Write;
use curl::easy::Easy;

const VALID_FILE_SIZE_LIMIT: f64 = 10000.0;

pub fn fetch_file(file_url: &str, out_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut out_file = std::fs::File::create(&out_path).expect("Error creating out_file");

    let mut easy = Easy::new();
    easy.url(file_url)?;
    easy.write_function(move |data| { 
        out_file.write_all(data).expect("Error writing data");
        Ok(data.len())
    })?;
    // let mut list = List::new();
    // list.append("Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==").unwrap();
    // easy.http_headers(list).unwrap();

    easy.perform()?;
    if easy.download_size()? < VALID_FILE_SIZE_LIMIT {
        std::fs::remove_file(out_path)?;
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "File too small to be valid")))
    } else { 
        Ok(()) 
    }
}


pub fn fetch_and_unzip(file_url: &str, out_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    
    let out_path = PathBuf::from(out_path); //Må gjøre sånn her så en &Path ikke borrowes inn i closuren under

    let mut easy = Easy::new();
    easy.url(file_url)?;
    easy.write_function(move |data| { 
        zip_extract::extract(std::io::Cursor::new(data), &out_path, true).expect("Unzipping error");
        Ok(data.len())
    })?;
    easy.perform()?;

    Ok(())
}