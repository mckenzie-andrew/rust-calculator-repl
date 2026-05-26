use std::io::{self, Write};

fn main() {

    loop {
        print!("calc> ");

        // Required to ensure our print shows up and
        // that stdout stays flushed after each iteration.
        io::stdout().flush().unwrap();
        let mut input = String::new();

        // To determine if we got an EOF or newline character
        // we have to store the result into a new variable.
        // 0 = EOF, anything > 0 bytes read
        let bytes_read = io::stdin()
            .read_line(&mut input)
            .expect("Invalid input.");

        if bytes_read == 0 {
            // EOF detected, gracefully exit loop.
            break;
        }

        let trimmed = input.trim().to_lowercase();
        if trimmed == "quit" || trimmed == "exit" {
            break;
        }
        
        println!("{}", input.trim());
    }
}
