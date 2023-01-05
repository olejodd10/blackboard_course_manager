use super::BBContent;
use crate::bb_session::BBSession;
use std::path::Path;
use std::io::Cursor;
use std::thread::JoinHandle;

pub struct BBAttachment<'a, 'b> {
    pub content: &'a BBContent<'b>,
    pub id: String,
    pub filename: String,
    pub mimetype: String,
}

impl<'a, 'b> BBAttachment<'a, 'b> {
    pub fn vec_from_json_results(json: Vec<u8>, content: &'a BBContent<'b>) -> Result<Vec<BBAttachment<'a, 'b>>, Box<dyn std::error::Error>> {
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

    pub fn download(&self, session: &BBSession, out_path: &Path, threads: &mut Vec<JoinHandle<f64>>) -> Result<(), Box<dyn std::error::Error>> {
        
        let url = format!("https://{}/learn/api/public/v1/courses/{}/contents/{}/attachments/{}/download",
        session.domain,
        self.content.course.id,
        self.content.id,
        self.id);
        
        let session = session.clone(); // Session is quite cheap to clone. session: Arc<BBSession> might be slightly faster. Maybe even Rc<BBSession> works, since it's never sent?
        let out_path = std::path::PathBuf::from(out_path);
        let is_zip = self.mimetype == "application/zip";
        threads.push(std::thread::spawn(move || {
            if is_zip { 
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
            } else {
                println!("Downloading {:?}", out_path.file_name().unwrap());
                session.download_file(&url, &out_path).unwrap()
            }
        }));
        Ok(())
    }

    pub fn view(&self) {
        println!("FILENAME: {}\nMIMETYPE: {}\n", self.filename, self.mimetype)
    }
}