use super::BBContent;
use std::path::Path;
use std::io::Cursor;
use std::thread::JoinHandle;

pub struct BBAttachment<'a, 'b, 'c> {
    pub content: &'a BBContent<'b, 'c>,
    pub id: String,
    pub filename: String,
    pub mimetype: String,
}

impl<'a, 'b, 'c> BBAttachment<'a, 'b, 'c> {
    pub fn vec_from_json_results(json: Vec<u8>, content: &'a BBContent<'b, 'c>) -> Result<Vec<BBAttachment<'a, 'b, 'c>>, Box<dyn std::error::Error>> {
        let json_string = std::string::String::from_utf8(json)?;
        let parsed_json = json::parse(&json_string)?;

        Ok(parsed_json["results"].members().map(|member| {
            BBAttachment {
                content,
                id: member["id"].to_string(),
                filename: member["fileName"].to_string(),
                mimetype: member["mimeType"].to_string(),           
            }
        }).collect())
    }

    
    pub fn download(&self, out_path: &Path, unzip: bool, overwrite: bool, threads: &mut Vec<JoinHandle<f64>>) -> Result<(), Box<dyn std::error::Error>> {
        
        let url = format!("https://{}/learn/api/public/v1/courses/{}/contents/{}/attachments/{}/download",
        self.content.course.manager.session.domain,
        self.content.course.id,
        self.content.id,
        self.id);
        
        let session = self.content.course.manager.session.clone();
        let out_path = std::path::PathBuf::from(out_path);
        let is_zip = self.mimetype == "application/zip";
        threads.push(std::thread::spawn(move || {
            if is_zip && unzip && (overwrite || !out_path.with_extension("").exists()) { 
                println!("Downloading and unzipping {:?}", out_path.file_name().unwrap());
                let bytes = session.download_bytes(&url).unwrap();
                let download_size = bytes.len() as f64;
                let out_dir = out_path.with_extension("");
                let unzip_result = zip_extract::extract(Cursor::new(bytes), &out_dir, true); // zip_extract explicitly wants &PathBuf
                if unzip_result.is_ok() { 
                    download_size // Consider returning size of unzipped folder
                } else {
                    eprintln!("Note: Unzipping of {:?} failed", out_path);
                    0.0
                }
            } else if overwrite || !out_path.exists() {
                println!("Downloading {:?}", out_path.file_name().unwrap());
                session.download_file(&url, &out_path).unwrap()
            } else {
                println!("Skipping download of {:?}", out_path.file_name().unwrap());
                0.0
            }
        }));
        Ok(())
    }

    pub fn view(&self) {
        println!("FILENAME: {}\nMIMETYPE: {}\n", self.filename, self.mimetype)
    }
}