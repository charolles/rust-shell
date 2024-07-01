use std::env;
use std::fs;
use std::io::Read;
use colored::Colorize;

pub fn cd(args: &[&str]) {
    let new_dir = if args.is_empty() {
        dirs::home_dir().unwrap_or_else(|| "/".into())
    } else {
        args[0].into()
    };
    match env::set_current_dir(&new_dir) {
        Ok(_) => {},
        Err(e) => eprintln!("cd: {}", e),
    }
}

pub fn echo(args: &[&str]) {
    println!("{}", args.join(" ").replace("\\n", "\n").replace("\\t", "\t"));
}

pub fn pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display().to_string().green()),
        Err(e) => eprintln!("pwd: {}", e),
    }
}

pub fn ls(args: &[&str]) {
    let path = if args.is_empty() {
        "."
    } else {
        args[0]
    };

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let metadata = entry.metadata().unwrap();
                        let file_name = entry.file_name();
                        if metadata.is_dir() {
                            println!("{}", file_name.to_string_lossy().blue());
                        } else {
                            println!("{}", file_name.to_string_lossy().white());
                        }
                    }
                    Err(e) => eprintln!("ls: {}", e),
                }
            }
        }
        Err(e) => eprintln!("ls: {}", e),
    }
}

pub fn touch(args: &[&str]) {
    for arg in args {
        match std::fs::File::create(arg) {
            Ok(_) => {},
            Err(e) => eprintln!("touch: {}: {}", arg, e),
        }
    }
}

pub fn mkdir(args: &[&str]) {
    for arg in args {
        match std::fs::create_dir(arg) {
            Ok(_) => {},
            Err(e) => eprintln!("mkdir: {}: {}", arg, e),
        }
    }
}

pub fn rm(args: &[&str]) {
    for arg in args {
        match std::fs::remove_file(arg) {
            Ok(_) => {},
            Err(e) => eprintln!("rm: {}: {}", arg, e),
        }
    }
}

pub fn cp(args: &[&str]) {
    if args.len() < 2 {
        eprintln!("Usage: cp <source> <destination>");
        return;
    }

    let source = args[0];
    let destination = args[1];

    match fs::copy(source, destination) {
        Ok(_) => {},
        Err(e) => eprintln!("cp: {} to {}: {}", source, destination, e),
    }
}

pub fn mv(args: &[&str]) {
    if args.len() < 2 {
        eprintln!("Usage: mv <source> <destination>");
        return;
    }

    let source = args[0];
    let destination = args[1];

    match fs::rename(source, destination) {
        Ok(_) => {},
        Err(e) => eprintln!("mv: {} to {}: {}", source, destination, e),
    }
}

pub fn cat(args: &[&str]) {
    for arg in args {
        let mut file = match fs::File::open(arg) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("cat: {}: {}", arg, e);
                continue;
            },
        };

        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            eprintln!("cat: {}: {}", arg, e);
            continue;
        }

        print!("{}", contents);
    }
}
