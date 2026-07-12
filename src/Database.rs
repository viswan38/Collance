#![allow(dead_code)]
#![allow(unused_imports)]
use std::fs;
use std::fs::{File, OpenOptions};
use std::path::Path;
use serde_json::error::Category::Data;
use crate::account::Account;
use crate::timeline::Timeline;
use crate::monetary::Monetary;
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
                eprint!("x{}", e);
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

    pub fn account_file_reader(&self) -> Vec<Account> {
        let contents = fs::read_to_string(Self::ACCOUNT_FILE);
        match contents {
            Ok(text) => {
                if text.trim().is_empty(){
                    Vec::new()
                }
                else{
                    serde_json::from_str(&text).unwrap_or_else(|_| Vec::new())
                }
            }

            Err(_) => Vec::new(),
        }
    }

    pub fn timeline_file_reader(&self) -> Vec<Timeline> {
        let contents = fs::read_to_string(Self::MONETARY_FILE);
        match contents {
            Ok(text) => {
                if text.trim().is_empty() {
                    Vec::new()
                }
                else {
                    serde_json::from_str(&text).unwrap_or_else(|_| Vec::new())
                }
            }
            Err(_) => Vec::new(),
        }
    }

    pub fn monetary_file_reader(&self) -> Vec<Monetary> {
        let contents = fs::read_to_string(Self::MONETARY_FILE);
        match contents {
            Ok(text) => {
                if text.trim().is_empty() {
                    Vec::new()
                }
                else{
                    serde_json::from_str(&text).unwrap_or_else(|_| Vec::new())
                }
            }
            Err(_) => Vec::new()
        }
    }

    pub fn write_to_account_f(&self, account: Account){
        let mut accounts = self.account_file_reader();
        accounts.push(account);
        let json = serde_json::to_string_pretty(&accounts).unwrap();
        std::fs::write(Self::ACCOUNT_FILE, json).expect("Account file was not made")
    }

    pub fn write_to_timeline_f(&self, timeline: Timeline) {
        let mut time = self.timeline_file_reader();
        time.push(timeline);
        let unwrap = serde_json::to_string_pretty(&time).unwrap();
        std::fs::write(Self::TIMELINE_FILE, unwrap).expect("Timeline file was not made")
    }

    pub fn write_to_monetary_f(&self, monetary: Monetary) {
        let mut money = self.monetary_file_reader();
        money.push(monetary);
        let unwrap = serde_json::to_string_pretty(&money).unwrap();
        std::fs::write(Self::MONETARY_FILE, unwrap).expect("Monetary file was not made")
    }

    pub fn remove_account(&self, account: &Account) -> bool {
        let mut accounts = self.account_file_reader();
        let first = accounts.len();
        accounts.retain(|a| !a.compare(account));

        if accounts.len() == first {
            return false;
        }

        let unwrap = serde_json::to_string_pretty(&accounts).unwrap();
        std::fs::write(Self::ACCOUNT_FILE, unwrap).expect("Could not rewrite account file");
        
        return true;
    }

    pub fn remove_timeline(&self, timeline: &Timeline) -> bool {
        let mut time = self.timeline_file_reader();
        let first = time.len();
        timeline.retain(|a| !a.compare(timeline));

        if time.len() == first {
            return false;
        }

        let unwrap = serde_json::to_string_pretty(&time).unwrap();
        std::fs::write(Self::TIMELINE_FILE, unwrap).expect("Timline file was not made");
        return true;
    }

    

}

