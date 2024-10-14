use tracing::info;
use crate::error::Error;

mod token_type;
mod token;
mod scanner;


pub fn run(lox_src: &str) -> Result<(), Error> {
    info!("Running {}", lox_src);
    Ok(())
}