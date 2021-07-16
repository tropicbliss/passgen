use clipboard::{ClipboardContext, ClipboardProvider};
use colored::Colorize;
use std::{
    error,
    fs::OpenOptions,
    io::{self, Write},
};

pub fn print_version() {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    println!("{}", VERSION);
}

pub fn print_generated_password(password: &str) {
    println!("{}{}", "Generated Password: ".blue(), password.bold());
}

pub fn copy_to_clipboard(password: &str) -> Result<(), Box<dyn error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(password.to_owned())?;
    Ok(())
}

pub fn clipboard_success_prompt() {
    println!("{}", "Password copied to clipboard".yellow());
}

pub fn save_password(password: &str) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("passwords.txt")?;
    let write_data = format!("{}{}", password, '\n');
    file.write_all(write_data.as_bytes())?;
    Ok(())
}

pub fn save_success_prompt() {
    println!("{}", "Password saved to passwords.txt".green());
}
