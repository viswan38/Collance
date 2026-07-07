mod account;

use crate::account::Account;

fn main() {
    let mut acc = Account::new(100);

    acc.deposit(50);

    println!("Balance: {}", acc.balance);
}