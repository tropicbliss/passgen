use argh::FromArgs;
use rand::{thread_rng, Rng};

#[derive(FromArgs)]
/// Simple Password Generator
pub struct Args {
    /// output the version number
    #[argh(switch, short = 'V')]
    version: bool,

    /// length of password (default: "8")
    #[argh(option, short = 'l', default = "8")]
    length: usize,

    /// save password to passwords.txt
    #[argh(switch, short = 's')]
    pub save: bool,

    /// remove numbers
    #[argh(switch)]
    no_numbers: bool,

    /// remove symbols
    #[argh(switch)]
    no_symbols: bool,
}

impl Args {
    pub fn parse_args() -> Option<Self> {
        let options: Args = argh::from_env();
        if options.version {
            None
        } else {
            Some(options)
        }
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
