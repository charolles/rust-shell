use std::process::Command;
use std::io::{self, Write};  // Write トレイトをインポート

pub fn execute_command(command: &str, args: &[&str]) -> io::Result<()> {
    let output = Command::new(command)
        .args(args)
        .output()?;

    if output.status.success() {
        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Command '{}' failed: {}", command, output.status)
        ))
    }
}