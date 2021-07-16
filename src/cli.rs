use ansi_term::Colour::{Blue, Green, Yellow};
use ansi_term::Style;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::{
    error,
    fs::OpenOptions,
    io::{self, Write},
};

pub struct CLI {
    password: String,
}

impl CLI {
    pub fn new(password: String) -> Self {
        Self { password }
    }

    pub fn print_generated_password(&self) {
        println!(
            "{}{}",
            Blue.paint("Generated Password: "),
            Style::new().bold().paint(&self.password)
        );
    }

    pub fn copy_to_clipboard(&self) -> Result<(), Box<dyn error::Error>> {
        let mut ctx: ClipboardContext = ClipboardProvider::new()?;
        ctx.set_contents(self.password.to_owned())?;
        Ok(())
    }

    pub fn clipboard_success_prompt(&self) {
        println!("{}", Yellow.paint("Password copied to clipboard"));
    }

    pub fn save_password(&self) -> Result<(), io::Error> {
        #[cfg(windows)]
        const LINE_ENDING: &str = "\r\n";
        #[cfg(not(windows))]
        const LINE_ENDING: &str = "\n";
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("passwords.txt")?;
        let write_data = format!("{}{}", self.password, LINE_ENDING);
        file.write_all(write_data.as_bytes())?;
        Ok(())
    }

    pub fn save_success_prompt(&self) {
        println!("{}", Green.paint("Password saved to passwords.txt"));
    }
}
