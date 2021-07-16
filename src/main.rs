mod cli;
mod utils;

fn main() {
    let options = utils::Args::parse_args();

    // Get generated password
    let password = options.generate_password();

    let cli = cli::Password::new(password);

    // Output generated password
    cli.print_generated_password();

    // Copy to clipboard
    cli.copy_to_clipboard();

    cli::clipboard_success_prompt();
    if options.save {
        // Save to file
        cli.save_password().expect("error saving password to file");

        cli::save_success_prompt();
    }
}
