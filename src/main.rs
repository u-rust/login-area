use std::io;
fn main() {
    let login = "alex";
    let password = "shit";

    loop {
        println!("Enter your login:");
        let mut input_login = String::new();
        io::stdin().read_line(&mut input_login).expect("Failed to read line");

        println!("Enter your password:");
        let mut input_password = String::new();
        io::stdin().read_line(&mut input_password).expect("Failed to read line");

        if input_login.trim().to_lowercase() == login && input_password.trim().to_lowercase() == password {
            println!("Login successful! Welcome, {}!", login);
            break;
        } else {
            println!("Incorrect login or password. Please try again.");
        }
    }
}