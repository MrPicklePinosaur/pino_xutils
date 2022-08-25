
use std::process::Command;
use thiserror::Error;
use anyhow::{Result, Error};

#[derive(Error, Debug)]
pub enum XrdbError {
    #[error("xrdb binary not found, are you sure you have it installed?")]
    XrdbMissing,
    #[error("xrdb exited with error: {0}")]
    XrdbErrored(String),
}

pub fn xrdb() -> Result<String> {
    let output = Command::new("xrdb")
        .arg("-query")
        .output()
        .map_err(|_| XrdbError::XrdbMissing)?;

    if !output.status.success()  {
        let error_str = String::from_utf8(output.stderr)?;
        return Err(Error::new(XrdbError::XrdbErrored(error_str)));
    }

    let output_str = String::from_utf8(output.stdout)?;
    Ok(output_str)
}

#[cfg(test)]
mod tests {
    use crate::xrdb;

    #[test]
    fn command(){
        println!("{}", xrdb().unwrap());
    }
}
