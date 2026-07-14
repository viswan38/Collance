use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    username: String,
    password: String,
}

impl Account {
    pub fn new(username: String, password: String) -> Self {
        Self {username,password,}
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
        
    }
    pub fn compare_account(&self, other : &Account) -> bool {
        if self.username == other.get_username(){
            if self.password == other.get_password(){
                return true; 
            }
        }

        return false;
    }

    //Work in progress, Next stage 
    //pub fn compare_timeline(&self, a : &Timeline) -> bool {}

    pub fn printing(&self) -> i32 {
        let x = 10;
        print!("hello world");
        return x;
    }
}