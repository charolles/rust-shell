use std::env;

pub fn change_directory(args: &[&str]) {
    let new_dir = args.get(0).map_or("/", |s| *s);
    match env::set_current_dir(new_dir) {
        Ok(_) => {},
        Err(e) => eprintln!("cd: {}", e),
    }
}

pub fn echo(args: &[&str]) {
    println!("{}", args.join(" "));
}

pub fn print_working_directory() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("pwd: {}", e),
    }
}