#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs;
use std::fs::{File, OpenOptions};
use std::path::Path;

use crate::account::Account;
use crate::timeline::Timeline;
use crate::monetary::Monetary;

pub struct Database;

impl Database {

    const ACCOUNT_FILE: &'static str = "Account_Information.json";
    const TIMELINE_FILE: &'static str = "Timeline.json";
    const MONETARY_FILE: &'static str = "Monetary.json";

    pub fn new() -> Self {
        Self::create_file(Self::ACCOUNT_FILE, "Account file error");
        Self::create_file(Self::TIMELINE_FILE, "Timeline file error");
        Self::create_file(Self::MONETARY_FILE, "Monetary file error");

        Database
    }

    fn create_file(file_name: &str, error_message: &str) {
        if !Path::new(file_name).exists() {
            if let Err(e) = File::create(file_name) {
                eprintln!("{}", error_message);
                eprintln!("{}", e);
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

    pub fn timeline_file_reader(&self) -> Vec<Timeline> {
        let contents = fs::read_to_string(Self::TIMELINE_FILE);
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
                else {
                    serde_json::from_str(&text).unwrap_or_else(|_| Vec::new())
                }
            }

            Err(_) => Vec::new(),
        }
    }

    pub fn write_to_account_f(&self, account: Account) {
        let mut accounts = self.account_file_reader();
        accounts.push(account);

        let json = serde_json::to_string_pretty(&accounts).unwrap();

        fs::write(Self::ACCOUNT_FILE, json).expect("Could not write account file");
    }

    pub fn write_to_timeline_f(&self, timeline: Timeline) {
        let mut time = self.timeline_file_reader();
        time.push(timeline);

        let json = serde_json::to_string_pretty(&time).unwrap();

        fs::write(Self::TIMELINE_FILE, json).expect("Could not write timeline file");
    }

    pub fn write_to_monetary_f(&self, monetary: Monetary) {
        let mut money = self.monetary_file_reader();
        money.push(monetary);

        let json = serde_json::to_string_pretty(&money).unwrap();

        fs::write(Self::MONETARY_FILE, json).expect("Could not write monetary file");
    }

    pub fn account_exists(&self, account: &Account) -> bool {
        let accounts = self.account_file_reader();

        for acc in accounts {
            if acc.compare_account(account) {
                return true;
            }
        }

        false
    }

    pub fn remove_account(&self, account: &Account) -> bool {
        let mut accounts = self.account_file_reader();
        let first = accounts.len();
        accounts.retain(|a| !a.compare_account(account));

        if accounts.len() == first {
            return false;
        }

        let unwrap = serde_json::to_string_pretty(&accounts).unwrap();
        std::fs::write(Self::ACCOUNT_FILE, unwrap).expect("Could not rewrite account file");
        
        return true;
    }
    /* Work in progress, next stage 
    pub fn remove_timeline(&self, timeline: &Timeline) -> bool {
        let mut time = self.timeline_file_reader();
        let first = time.len();
        time.retain(|a| !a.compare(timeline));

        if time.len() == first {
            return false;
        }

        let unwrap = serde_json::to_string_pretty(&time).unwrap();
        std::fs::write(Self::TIMELINE_FILE, unwrap).expect("Timline file was not made");
        return true;
    }
    */
    

}

