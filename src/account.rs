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
    pub fn compare(&self, a : &Account) -> bool {
        if(self.username == a.get_username()){
            if(self.password == a.get_password()){
                return true;
            }
        }
        return false;
    }
    pub fn printing(&self) -> i32 {
        let x = 10;
        print!("hello world");
        return x;
    }
}