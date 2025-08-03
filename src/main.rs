use std::{self, io};

fn main() {
    let mut text = String::new();

    loop {
        println!("SomeScript :> ");
        io::stdin()
            .read_line(&mut text)
            .expect("Failed to readline"); // Do proper error handling

        text = text.trim().to_string();

        if ".quit()" == text.to_lowercase().as_str() {
            break;
        }
    }
}
