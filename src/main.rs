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
    
    // Get generated password
    let password = options.generate_password();
    
    // Output generated password
    cli::print_generated_password(&password);
    
    // Copy to clipboard
    cli::copy_to_clipboard(&password).expect("error copying to clipboard");
    
    cli::clipboard_success_prompt();
    if options.save {
        
        // Save to file
        cli::save_password(&password).expect("error saving password");
        
        cli::save_success_prompt();
    }
}
