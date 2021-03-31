extern crate toml;
extern crate dirs;

use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Credentials {
    registry: Registry
}

#[derive(Deserialize)]
struct Registry {
    token: String
}

fn main() {

    // Get the credential path
    let mut creds_file_path = dirs::home_dir().unwrap();
    creds_file_path.push(".cargo");
    creds_file_path.push("credentials");
    let creds_file_path = creds_file_path.to_str().unwrap();

    // Load the credentials file
    let creds_file = fs::read_to_string(&creds_file_path).unwrap();

    // Parse TOML
    let creds : Credentials = toml::from_str(&creds_file).unwrap();

    // Print the token
    println!("{}", creds.registry.token);
    
}
