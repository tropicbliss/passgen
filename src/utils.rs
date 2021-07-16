use rand::{thread_rng, Rng};
use structopt::StructOpt;

/// Simple Password Generator
#[derive(StructOpt, Debug)]
#[structopt(name = "passgen")]
pub struct Args {
    /// length of password
    #[structopt(short, long, default_value = "8")]
    length: usize,

    /// save password to passwords.txt
    #[structopt(short, long)]
    pub save: bool,

    /// remove numbers
    #[structopt(long)]
    no_numbers: bool,

    /// remove symbols
    #[structopt(long)]
    no_symbols: bool,
}

impl Args {
    pub fn parse_args() -> Self {
        Args::from_args()
    }

    pub fn generate_password(&self) -> String {
        let length = self.length;
        let char_array = self.generate_char_array();
        let mut rng = thread_rng();
        let password: String = (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..char_array.len());
                char_array[idx] as char
            })
            .collect();
        password
    }

    fn generate_char_array(&self) -> Vec<u8> {
        const ALPHA: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        const NUMBERS: &str = "0123456789";
        const SYMBOLS: &str = "!@#$%^&*_-+=";
        let has_numbers = !self.no_numbers;
        let has_symbols = !self.no_symbols;
        let mut chars = ALPHA.to_string();
        if has_numbers {
            chars.push_str(NUMBERS)
        };
        if has_symbols {
            chars.push_str(SYMBOLS)
        };
        chars.into_bytes()
    }
}
