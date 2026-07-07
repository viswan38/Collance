pub struct Account {
    pub balance: i32,
}

impl Account {
    pub fn new(balance: i32) -> Self {
        Self { balance }
    }

    pub fn deposit(&mut self, amount: i32) {
        print!("5");
        self.balance += amount;
    }
}