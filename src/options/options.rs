use structopt::StructOpt;
use std::str::FromStr;
use crate::options::password_length::PasswordLength;

#[derive(StructOpt, Debug)]
#[structopt(about = "A password generation tool", author = "Artem Korobko")]
pub struct Options {
    #[structopt(short, long, possible_values = &PasswordLength::variants(),
    case_insensitive = true, default_value = "r", help = "Password length")]
    pub length: PasswordLength
}

impl Options {
    pub fn from_args() -> Self {
        <Self as StructOpt>::from_args()
    }
}
