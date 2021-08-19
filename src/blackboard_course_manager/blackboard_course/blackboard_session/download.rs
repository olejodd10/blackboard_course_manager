use std::path::{Path, PathBuf};
use std::io::Write;
use curl::easy::{Easy, List};


const PATH_LENGTH_WARNING_LIMIT: usize = 230;


pub fn download_file(file_url: &str, out_path: &Path, cookie_file_path: Option<&Path>, overwrite: bool) -> Result<f64, Box<dyn std::error::Error>> {

    if !overwrite && out_path.exists() { 
        if out_path.extension().unwrap() != "json" {
            eprintln!("Skipping download of {}", out_path.file_name().unwrap().to_str().unwrap());
        }
        return Ok(0.0); 
    }

    if out_path.extension().unwrap() != "json" {
        eprintln!("Downloading {}", out_path.file_name().unwrap().to_str().unwrap());
    }

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


pub fn download_and_unzip(file_url: &str, out_path: &Path, cookie_file_path: Option<&Path>, overwrite: bool) -> Result<f64, Box<dyn std::error::Error>> {

    let out_dir = out_path.with_extension(""); //Må gjøre sånn her så en &Path ikke borrowes inn i closuren under
    
    if out_dir.exists() {
        if !overwrite {
            eprintln!("Skipping download of {} because unzipped folder already exists", out_dir.file_name().unwrap().to_str().unwrap());
            return Ok(0.0);
        } else {
            std::fs::remove_dir_all(&out_dir)?;
            std::fs::create_dir_all(&out_dir)?; // Må klone for å unngå feilmelding        
        }
    } 

    let download_size = download_file(file_url, out_path, cookie_file_path, overwrite)?;

    eprintln!("Unzipping {:?}", out_path);

    let zip_file = std::fs::File::open(out_path)?;
    let unzip_result = zip_extract::extract(zip_file, &out_dir, true);

    if unzip_result.is_ok() {
        std::fs::remove_file(&out_path)?;
    } else {
        eprintln!("Note: Unzipping of {:?} failed", &out_path);
    }

    Ok(download_size) // Consider displaying extracted file size
}
