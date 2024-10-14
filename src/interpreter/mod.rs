use tracing::info;

use crate::error::Error;

pub fn run(lox_src: &str) -> Result<(), Error> {
    info!("Running {}", lox_src);
    Ok(())
}