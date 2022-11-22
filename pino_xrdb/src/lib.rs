//! Rust wrapper for `xrdb`. Query the system's xrdb or add new values to it. Can handle wildcards
//! resources.
//!
//! ```rust
//! use pino_xrdb::Xrdb;
//! fn main() {
//!
//!     let mut xrdb = Xrdb::new();
//!     xrdb.read().unwrap();
//!     
//!     if let Some(value) = xrdb.query("dwm", "color1") {
//!         println!("dwm.color1 has value {}", value);
//!     } else {
//!         println!("dwm.color1 not found");
//!     }
//!     
//! }
//! ```

use std::collections::HashMap;
use std::process::Command;

/// Error types for xrdb
#[derive(Debug)]
pub enum XrdbError {
    /// The xrdb executable was not found, you should install it
    Missing,
    /// xrdb exited with error
    Errored(String),
    /// xrdb output was invalid
    Invalid,
    /// xrdb output was not able to be parsed as string
    OutputMalformed,
}

impl std::error::Error for XrdbError {}

impl std::fmt::Display for XrdbError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            XrdbError::Missing => write!(f, "xrdb binary not found, are you sure you have it installed?"),
            XrdbError::Errored(e) => write!(f, "xrdb exited with error: {0}", e),
            XrdbError::Invalid => write!(f, "failed to parse line"),
            XrdbError::OutputMalformed => write!(f, "could not parse xrdb output to string")
        }
    }
}

/// Xrdb database struct
#[derive(Default)]
pub struct Xrdb {
    db: HashMap<String, HashMap<String, String>>,
    univeral: HashMap<String, String> 
}

impl Xrdb {
    
    /// Construct a new Xrdb database
    pub fn new() -> Self {
        Xrdb::default()
    }

    /// Read system xrdb
    ///
    /// ```rust
    /// # use pino_xrdb::Xrdb;
    /// # fn main() {
    /// let mut xrdb = Xrdb::new();
    /// xrdb.read().unwrap();
    /// # }
    /// ```
    pub fn read(&mut self) -> Result<(), XrdbError> {

        // run xrdb command 
        let output = Command::new("xrdb")
            .arg("-query")
            .output()
            .map_err(|_| XrdbError::Missing)?;

        if !output.status.success()  {
            let error_str = String::from_utf8(output.stderr).map_err(|_| XrdbError::OutputMalformed)?;
            return Err(XrdbError::Errored(error_str));
        }

        let output_str = String::from_utf8(output.stdout).map_err(|_| XrdbError::OutputMalformed)?;

        // parse output
        for line in output_str.lines() {
            let (prog, rest) = match line.split_once('.') {
                Some(x) => x,
                None => continue
            };
            let (res, val) = match rest.split_once(':') {
                Some(x) => x,
                None => continue
            };

            if prog.trim() == "*" {
                self.insert_universal(res.trim(), val.trim());
            } else {
                self.insert(prog.trim(), res.trim(), val.trim());
            }        
        }

        Ok(())
    }

    /// Insert a new resource
    ///
    /// Inserting a resource that already exists will replace it.
    /// ```rust
    /// # use pino_xrdb::Xrdb;
    /// # fn main() {
    /// let mut xrdb = Xrdb::new();
    /// xrdb.insert("dwm", "color1", "#ea6962");
    /// 
    /// assert_eq!(xrdb.query("dwm", "color1"), Some(String::from("#ea6962")));
    /// # }
    /// ```
    pub fn insert(&mut self, program: &str, res: &str, val: &str) {
        self.get_prog_mut(program).insert(res.into(), val.into());
    }

    /// Insert a universal resource.
    ///
    /// Inserting a universal resource that already exists will replace it. Program specific
    /// resources will not be overwritten.
    /// ```rust
    /// # use pino_xrdb::Xrdb;
    /// # fn main() {
    /// let mut xrdb = Xrdb::new();
    /// xrdb.insert_universal("color1", "#ea6962");
    /// 
    /// assert_eq!(xrdb.query("dwm", "color1"), Some(String::from("#ea6962")));
    /// assert_eq!(xrdb.query("st", "color1"), Some(String::from("#ea6962")));
    /// assert_eq!(xrdb.query("dmenu", "color1"), Some(String::from("#ea6962")));
    /// # }
    /// ```
    pub fn insert_universal(&mut self, res: &str, val: &str) {
        self.univeral.insert(res.into(), val.into());
    }

    /// Query a given resource
    ///
    /// If a resource was not defined for a given program, query will return the universal
    /// resource. In the case that a resource was specifically defined for that program (via
    /// [Xrdb::insert]), the program specific resource will be returned.
    ///
    /// ```rust
    /// # use pino_xrdb::Xrdb;
    /// # fn main() {
    /// let mut xrdb = Xrdb::new();
    /// xrdb.insert_universal("color1", "#ea6962");
    /// 
    /// assert_eq!(xrdb.query("dwm", "color1"), Some(String::from("#ea6962")));
    /// assert_eq!(xrdb.query("st", "color1"), Some(String::from("#ea6962")));
    /// assert_eq!(xrdb.query("dmenu", "color1"), Some(String::from("#ea6962")));
    ///
    /// xrdb.insert("dwm", "color1", "#ffffff");
    /// assert_eq!(xrdb.query("dwm", "color1"), Some(String::from("#ffffff")));
    /// assert_eq!(xrdb.query("st", "color1"), Some(String::from("#ea6962")));
    /// assert_eq!(xrdb.query("dmenu", "color1"), Some(String::from("#ea6962")));
    /// # }
    /// ```
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
