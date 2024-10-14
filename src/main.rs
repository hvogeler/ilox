use std::{fs::File, io::{self, Read, Write}, path::Path};
use clap::Parser;
use error::Error;
use interpreter::run;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod interpreter;
mod error;

#[derive(Parser, Debug, Clone)]
#[command(version)]
struct Args {
    script: Option<String>,
}

fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .with_line_number(true)
    .with_thread_ids(true)
    .with_thread_names(true)
    .with_target(true)
    // .pretty()
    .init();

    let args = Args::parse();
    if let Some(script) = args.script {
        info!("Script: {}", script);
        let result = run_file(Path::new(&script));
        if let Err(e) = result {
            tracing::error!("{:?}", e);
        }
    } else {
        let result = repl();
        if let Err(e) = result {
            tracing::error!("{:?}", e);
        }
    }
}

fn run_file(file: &Path) -> Result<(), Error> {
    let mut buf = String::new();
    let mut fd = File::open(file)?;
    File::read_to_string(&mut fd, &mut buf)?;
    run(&buf)?;
    Ok(())
}

fn repl() -> Result<(), Error> {

    let mut buf = String::new();
    prompt()?;
    let mut line_len = io::stdin().read_line(&mut buf)?;
    while line_len > 0 {
        let line = &buf;
        if line.to_lowercase().starts_with("quit") || line.to_lowercase().starts_with("qq") { 
            break;
        }
        run(&line)?;
        buf.clear();
        prompt()?;
        line_len = io::stdin().read_line(&mut buf)?;
    }
    Ok(())
}

fn prompt() -> Result<(), Error> {
    io::stdout().write_all(b"> ")?;
    io::stdout().flush()?;
    Ok(())
}