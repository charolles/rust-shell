mod commands;

use std::io::{self, Write};
use commands::{builtin};

fn main() {
    loop {
        print!("rsh> ");
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
            "cd" => builtin::cd(&args),
            "echo" => builtin::echo(&args),
            "pwd" => builtin::pwd(),
            "ls" => builtin::ls(&args),
            "touch" => builtin::touch(&args),
            "mkdir" => builtin::mkdir(&args),
            "rm" => builtin::rm(&args),
            "cp" => builtin::cp(&args),
            "mv" => builtin::mv(&args),
            "cat" => builtin::cat(&args),
            _ => {
                eprintln!("Error: '{}' is not a recognized built-in command.", command);
            }
        }
    }
}
