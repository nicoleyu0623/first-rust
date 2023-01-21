use std::{fs, io};
use std::io::Read;
use thiserror::Error;

#[derive(Error, Debug)]
enum ReadUsernameError {

    #[error("Could not read: {0}")]
    IoError(#[from] io::Error),

    // Error: Found no username in <config.dat>
    #[error("Found no username in {0}")]
    EmptyUsername(String),

}

fn read_username(path: &str) -> Result<String, ReadUsernameError> {
    let mut username = String::with_capacity(100);
    fs::File::open(path)?.read_to_string(&mut username)?;
    if username.is_empty() {
        return Err(ReadUsernameError::EmptyUsername(String::from(path)));
    }
    Ok(username)
}

fn main() {
    fs::write("config.dat", "").unwrap();
    match read_username("config.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err) => println!("Error: {err}"),
    }
}