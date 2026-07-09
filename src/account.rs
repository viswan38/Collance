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
    pub fn printing(&self) -> i32 {
        let x = 10;
        print!("hello world");
        return x;
    }
}