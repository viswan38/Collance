#![allow(dead_code)]
#![allow(unused_imports)]
use std::fs::{File, OpenOptions};
use std::path::Path;

use serde_json::error::Category::Data;

pub struct Database;

impl Database{

    const ACCOUNT_FILE: &'static str = "Account_Information.ser";
    const TIMELINE_FILE: &'static str = "Timeline.ser";
    const MONETARY_FILE: &'static str = "Monetary.ser";
    //const HOURS_FILE: &'static str = "Hours.txt";
    pub fn new() -> Self {
        Self::create_file(Self::ACCOUNT_FILE, "Account file error");
        Self::create_file(Self::TIMELINE_FILE, "Timeline file error");
        Self::create_file(Self::MONETARY_FILE, "Monetary file error");

        Database
    }

    fn create_file(file_name: &str, error_message: &str){
        if !Path::new(file_name).exists() {
            if let Err(e) = File::create(file_name) {
                eprint!("{}", error_message);
                eprint!("{}", e);
            }
        }
    }

    pub fn get_account_file(&self) -> &str {
        Self::ACCOUNT_FILE
    }
    pub fn get_timeline_file(&self) -> &str {
        Self::TIMELINE_FILE
    }
    pub fn get_monetary_file(&self) -> &str {
        Self::MONETARY_FILE
    }
    

}





