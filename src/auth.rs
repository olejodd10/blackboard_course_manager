use curl::easy::{Easy, List};
use std::path::PathBuf;
use std::io::Write;

pub fn get_authorization_code() {
    
    let app_id = "1c49c049-ccb1-4cff-a607-1abd1e69082c";
    let client_key = "16abf9e7-cc20-49d3-b36d-a2f8e5bb0c92";
    let client_secret = "1TPLnYNaDoYHmUtQ98y3cGwtnmROVMOJ";
    
    let response_type = "code";
    let redirect_uri = "http://google.com";
    
    let scope = "read";
    
    let url = format!("https://ntnu.blackboard.com//learn/api/public/v1/oauth2/authorizationcode?redirect_uri={}&response_type={}&client_id={}&scope={}",
        redirect_uri,
        response_type,
        app_id,
        scope);


    let mut handle = Easy::new();
    handle.url(&url).unwrap();

    let out_path = PathBuf::from("./test.txt");
    let mut out_file = std::fs::File::create(&out_path).expect("Error creating out_file");

    handle.show_header(true).unwrap();
    handle.write_function(move |data| { 
        out_file.write_all(data).expect("Error writing data");
        Ok(data.len())
    }).unwrap();
    handle.perform().unwrap();

    eprintln!("Response code: {}", handle.response_code().unwrap());

}