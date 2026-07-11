#![allow(dead_code)]
#![allow(unused_imports)]
use std::fs::{File, OpenOptions};
use std::path::Path;

pub struct Database;

impl Database{
    pub fn new() -> Self {
        const ACCOUNT_FILE: &'static str = "Account_Information.ser";
        const TIMELINE_FILE: &'static str = "Timeline.ser";
        const MONETARY_FILE: &'static str = "Monetary.ser";
        const HOURS_FILE: &'static str = "Hours.txt";
    }

    fn create_file(file_name: &str, error_message: &str){
        if(!Path::new(file_name).exists()){
            if(let Err(e) = File::create(file_name)){
                eprint!("{}", error_message);
                eprint!("{}", e);
            }
        }
    }

    pub fn get_account_file(&self) -> &str {
        Self::ACCOUNT_FILE
    }

    pub fn get_account_file(&self) -> &str {
        Self::ACCOUNT_FILE
    }
}





