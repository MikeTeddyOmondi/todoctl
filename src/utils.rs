use std::io::{self, Write};

/// Prompt the user and read a line
pub fn user_input(prompt: String) -> String {
    loop {
        println!("___________________________________");
        print!("Enter {}: ", prompt);
        let _ = io::stdout().flush();

        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_err() {
            eprintln!("Failed to read input; try again.");
            continue;
        }

        let val = buf.trim_end(); // keep leading/trailing whitespace if you want, but not newline
        if val.is_empty() {
            println!("⚠️  Input cannot be empty. Please try again.");
            continue;
        }

        println!("___________________________________");
        return val.to_string();
    }
}
