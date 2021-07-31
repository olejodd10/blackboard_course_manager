use std::path::{Path, PathBuf};
use std::io::Write;
use curl::easy::{Easy, List};


pub fn download_file(file_url: &str, out_path: &Path, access_token: Option<&str>) -> Result<f64, Box<dyn std::error::Error>> {
    
    let mut out_file = std::fs::File::create(&out_path).expect("Error creating out_file");

    let mut easy = Easy::new();
    easy.url(file_url)?;
    easy.write_function(move |data| { 
        out_file.write_all(data).expect("Error writing data");
        Ok(data.len())
    })?;

    if let Some(access_token) = access_token {
        let mut list = List::new();
        let autorization_header = format!("Authorization: Bearer {}", access_token);
        list.append(&autorization_header).unwrap();
        easy.http_headers(list).unwrap();
    }

    easy.perform()?;

    eprintln!("Response code: {}", easy.response_code().unwrap());

    Ok(easy.download_size()?)
}


pub fn download_and_unzip(file_url: &str, out_path: &Path, access_token: Option<&str>) -> Result<f64, Box<dyn std::error::Error>> {
    
    let out_path = PathBuf::from(out_path); //Må gjøre sånn her så en &Path ikke borrowes inn i closuren under

    let mut easy = Easy::new();
    easy.url(file_url)?;
    easy.write_function(move |data| { 
        zip_extract::extract(std::io::Cursor::new(data), &out_path, true).expect("Unzipping error");
        Ok(data.len())
    })?;

    if let Some(access_token) = access_token {
        let mut list = List::new();
        let autorization_header = format!("Authorization: Bearer {}", access_token);
        list.append(&autorization_header).unwrap();
        easy.http_headers(list).unwrap();
    }

    eprintln!("Response code: {}", easy.response_code().unwrap());

    easy.perform()?;

    Ok(easy.download_size()?)
}
