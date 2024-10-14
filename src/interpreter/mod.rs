use scanner::Scanner;
use tracing::info;
use crate::error::Error;

mod token_type;
mod token;
mod scanner;


pub fn run(lox_src: &str) -> Result<(), Error> {
    let mut scanner = Scanner::new(lox_src);
    let tokens = scanner.scan_tokens();
    Ok(())
}