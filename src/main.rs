mod cli;
mod utils;

fn main() {
    let options = match utils::Args::parse_args() {
        Some(x) => x,
        None => {
            cli::print_version();
            std::process::exit(0);
        }
    };
    let password = options.generate_password();
    cli::print_generated_password(&password);
    cli::copy_to_clipboard(&password).expect("error copying to clipboard");
    cli::clipboard_success_prompt();
    if options.save {
        cli::save_password(&password).expect("error saving password");
        cli::save_success_prompt();
    }
}
