use std::path::Path;
use std::io::Write;
use curl::easy::Easy;


const PATH_LENGTH_WARNING_LIMIT: usize = 230;


pub fn download_file(file_url: &str, out_path: &Path, cookie_file_path: Option<&Path>) -> Result<f64, Box<dyn std::error::Error>> {

    if let Ok(absolute_path) = out_path.canonicalize() {
        if absolute_path.to_str().unwrap().len() > PATH_LENGTH_WARNING_LIMIT {
            eprintln!("WARNING: Path length exceeds {} characters, and might approach system limit.", PATH_LENGTH_WARNING_LIMIT);
        }
    } 
    
    let mut out_file = std::fs::File::create(&out_path).expect("Error creating out file");

    let mut easy = Easy::new();
    easy.url(file_url)?;
    easy.write_function(move |data| { 
        out_file.write_all(data).expect("Error writing data");
        Ok(data.len())
    })?;

    if let Some(cookie_file_path) = cookie_file_path {
        easy.cookie_file(cookie_file_path).unwrap();
    }

    easy.follow_location(true)?; //Viktig fordi BB redirecter (302)
    easy.fail_on_error(true)?; //Viktig for å faile på 401
    
    easy.perform()?;
    
    Ok(easy.download_size()?)
}