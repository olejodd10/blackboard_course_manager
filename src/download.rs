use std::path::{Path, PathBuf};
use std::io::Write;
use curl::easy::{Easy, List};


pub fn download_file(file_url: &str, out_path: &Path, headers: Option<&[&str]>, overwrite: bool) -> Result<f64, Box<dyn std::error::Error>> {
    
    if !overwrite && out_path.exists() { return Ok(0.0); }

    let mut out_file = std::fs::File::create(&out_path).expect("Error creating out_file");

    let mut easy = Easy::new();
    easy.url(file_url)?;
    easy.write_function(move |data| { 
        out_file.write_all(data).expect("Error writing data");
        Ok(data.len())
    })?;

    if let Some(headers) = headers {
        let mut list = List::new();
        for header in headers {
            // eprintln!("Adding header: {}", header);
            list.append(header).unwrap();
        }
        easy.http_headers(list).unwrap();
    }

    easy.follow_location(true)?; //Viktig fordi BB redirecter (302)
    easy.fail_on_error(true)?; //Viktig for å faile på 401
    
    easy.perform()?;
    
    // eprintln!("Response code: {}", easy.response_code().unwrap());

    Ok(easy.download_size()?)
}


pub fn download_and_unzip(file_url: &str, out_path: &Path, headers: Option<&[&str]>, overwrite: bool) -> Result<f64, Box<dyn std::error::Error>> {

    let out_dir = PathBuf::from(out_path.with_extension("")); //Må gjøre sånn her så en &Path ikke borrowes inn i closuren under
    
    if out_dir.exists() {
        if !overwrite {
            return Ok(0.0);
        } else {
            std::fs::remove_dir_all(&out_dir)?;
            std::fs::create_dir_all(out_dir.clone())?; // Må klone for å unngå feilmelding        
        }
    }

    let mut easy = Easy::new();
    easy.url(file_url)?;
    easy.write_function(move |data| { 
        zip_extract::extract(std::io::Cursor::new(data), &out_dir, true).expect("Unzipping error");
        Ok(data.len())
    })?;

    if let Some(headers) = headers {
        let mut list = List::new();
        for header in headers {
            // eprintln!("Adding header: {}", header);
            list.append(header).unwrap();
        }
        easy.http_headers(list).unwrap();
    }

    easy.follow_location(true)?; //Viktig fordi BB redirecter (302)
    easy.fail_on_error(true)?; //Viktig for å faile på 401

    easy.perform()?;

    Ok(easy.download_size()?)
}
