use ansi_term::Colour::{Blue, Green, Yellow};
use ansi_term::Style;
use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;
use std::{
    fs::OpenOptions,
    io::Write,
};
use anyhow::{bail, Result};

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
        let mut ctx = ClipboardContext::new().unwrap();
        match ctx.set_contents(self.password.clone()) {
            Ok(_) => Ok(()),
            Err(e) => bail!(e)
        }
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
