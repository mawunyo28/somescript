use std::{self, io};

use somescript::build;

fn main() {
    let mut text = String::new();

    loop {
        println!("SomeScript :> ");
        io::stdin()
            .read_line(&mut text)
            .expect("Failed to readline"); // Do proper error handling

        if ".quit()" == text.trim().to_lowercase().as_str() {
            break;
        } else {
            let text = text.clone();
            let result = build(text);

            println!("{:?}", result);
        }

        text.clear();
    }
}
