use curl::easy::{Easy, List};
use std::path::PathBuf;
use std::io::Write;

// 2LO Autentisering:
// https://docs.blackboard.com/learn/rest/getting-started/basic-authentication

// 3LO autentisering: 
// https://docs.blackboard.com/learn/rest/getting-started/3lo

// Autentisering eksempel med python:
// https://docs.blackboard.com/learn/rest/examples/python-demo

// Laste ned filer (med curl):
// https://docs.blackboard.com/learn/rest/examples/curl-attachments-demo

pub fn get_authorization_code() {
    
    let app_id = "1c49c049-ccb1-4cff-a607-1abd1e69082c";
    let client_key = "16abf9e7-cc20-49d3-b36d-a2f8e5bb0c92";
    let client_secret = "1TPLnYNaDoYHmUtQ98y3cGwtnmROVMOJ";
    
    let response_type = "code";
    let redirect_uri = "https://localhost";
    
    let scope = "read";
    
    let url = format!("https://ntnu.blackboard.com//learn/api/public/v1/oauth2/authorizationcode?redirect_uri={}&response_type={}&client_id={}&scope={}",
        redirect_uri,
        response_type,
        client_key,
        scope);


    let mut handle = Easy::new();
    handle.url(&url).unwrap();

    eprintln!("url: {}", url);

    let out_path = PathBuf::from("./result.txt");
    let mut out_file = std::fs::File::create(&out_path).expect("Error creating out_file");

    handle.show_header(true).unwrap();
    handle.write_function(move |data| { 
        out_file.write_all(data).expect("Error writing data");
        Ok(data.len())
    }).unwrap();
    handle.perform().unwrap();

    eprintln!("Response code: {}", handle.response_code().unwrap());

}

// pub fn get_tokeninfo() {
    
// }