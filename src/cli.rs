use ansi_term::Colour::{Blue, Green, Yellow};
use ansi_term::Style;
use anyhow::{anyhow, Result};
use clipboard_ext::prelude::*;
use clipboard_ext::x11_fork::ClipboardContext;
use std::{fs::OpenOptions, io::Write};

pub struct Password {
    password: String,
}

impl Password {
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

    pub fn copy_to_clipboard(&self) -> Result<()> {
        let mut ctx: ClipboardContext =
            ClipboardProvider::new().map_err(|_| anyhow!("Failed to open clipboard."))?;
        ctx.set_contents(self.password.clone())
            .map_err(|_| anyhow!("Failed to set the clipboard contents."))?;
        Ok(())
    }

    pub fn save_password(&self) -> Result<()> {
        #[cfg(windows)]
        const LINE_ENDING: &str = "\r\n";
        #[cfg(not(windows))]
        const LINE_ENDING: &str = "\n";
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("passwords.txt")?;
        let write_data = format!("{}{}", self.password, LINE_ENDING).into_bytes();
        file.write_all(&write_data)?;
        Ok(())
    }
}

pub fn clipboard_success_prompt() {
    println!("{}", Yellow.paint("Password copied to clipboard"));
}

pub fn save_success_prompt() {
    println!("{}", Green.paint("Password saved to passwords.txt"));
}
