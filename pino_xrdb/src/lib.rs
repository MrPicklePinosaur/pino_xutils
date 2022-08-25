
use std::collections::HashMap;
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

pub struct Xrdb {
    db: HashMap<String, HashMap<String, String>>,
    univeral: HashMap<String, String> 
}

impl Xrdb {

    // pub fn new() -> Result<Self> {
         
    // }

    /// Insert a new resource
    pub fn insert(&mut self, program: &str, res: String, val: String) {
        self.get_prog_mut(program).insert(res, val);
    }

    /// Insert a universal resource
    pub fn insert_univeral(&mut self, res: String, val: String) {
        self.univeral.insert(res, val);
    }

    /// Query a given resource
    pub fn query(&self, program: &str, res: &str) -> Option<String> {
        if let Some(prog) = self.db.get(program) {
            if let Some(val) = prog.get(res) {
                return Some(val.to_owned());
            }
        }
        
        // check if resource was defined in universal
        self.univeral.get(res).map(|v| v.to_owned())
    }

    /// Return reference to query table or creates it if not exist
    fn get_prog(&mut self, program: &str) -> &HashMap<String, String> {
        if !self.db.contains_key(program) {
            self.db.insert(program.to_owned(), HashMap::new());
        }
        self.db.get(program).unwrap()
    }

    /// Mutable version of [get_prog]
    fn get_prog_mut(&mut self, program: &str) -> &mut HashMap<String, String> {
        if !self.db.contains_key(program) {
            self.db.insert(program.to_owned(), HashMap::new());
        }
        self.db.get_mut(program).unwrap()
    }

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

pub fn parse_xrdb(input: &str) {
    
    for line in input.lines() {
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn command(){
        let output = xrdb().unwrap();
        parse_xrdb(&output);
    }
}
