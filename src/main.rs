#![warn(clippy::pedantic)]

mod cli;
mod utils;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let options = utils::Args::parse_args();

    // Get generated password
    let password = options.generate_password();

    let cli = cli::Password::new(password);

    // Copy to clipboard
    cli.copy_to_clipboard()
        .with_context(|| "Error copying to clipboard")?;

    // Output generated password
    cli.print_generated_password();
    cli::clipboard_success_prompt();

    // Save to file
    if options.save {
        cli.save_password()
            .with_context(|| "Failed to write password to passwords.txt")?;
        cli::save_success_prompt();
    }

    Ok(())
}
