#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod account;
mod Database;
mod Timeline;
mod timeline;
mod monetary;

fn main() {
    let account = account::Account::new(
        "Srivatsa".to_string(),
        "password123".to_string()
    );

    print!("{}", account.get_username());
    print!("{}", account.printing());
}