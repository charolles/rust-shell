mod commands;

use std::io::{self, Write};
use commands::{builtin, external};

fn main() {
    loop {
        print!("my_shell> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        match command {
            "exit" => break,
            "cd" => builtin::change_directory(&args),
            "echo" => builtin::echo(&args),
            "pwd" => builtin::print_working_directory(),
            _ => {
                if let Err(e) = external::execute_command(command, &args) {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}