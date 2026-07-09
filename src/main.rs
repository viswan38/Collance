mod account;

fn main() {
    let account = account::Account::new(
        "Srivatsa".to_string(),
        "password123".to_string()
    );

    print!("{}", account.get_username());
    print!("{}", account.printing());
    //print!("{}", account.set_password("Srivatsa".to_string()));
}