use clap::Parser;
use error::{Error, Errors};
use interpreter::run;
use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};
use tracing_subscriber::EnvFilter;

mod error;
mod interpreter;

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
        .pretty()
        .init();

    let args = Args::parse();

    let mut errors: Errors = Vec::new();
    if let Some(script) = args.script {
        let result = run_file(Path::new(&script), &mut errors);
        if let Err(e) = result {
            tracing::error!("{:?}", e);
        }
    } else {
        let result = repl(&mut errors);
        if let Err(e) = result {
            tracing::error!("{:?}", e);
        }
    }
}

fn run_file(file: &Path, errors: &mut Errors) -> Result<(), Error> {
    let mut buf = String::new();
    let mut fd = File::open(file)?;
    File::read_to_string(&mut fd, &mut buf)?;
    run(&buf, errors)?;
    Ok(())
}

fn repl(errors: &mut Errors) -> Result<(), Error> {
    let mut buf = String::new();
    prompt()?;
    let mut line_len = io::stdin().read_line(&mut buf)?;
    while line_len > 0 {
        let line = &buf;
        if line.to_lowercase().starts_with("quit") || line.to_lowercase().starts_with("qq") {
            break;
        }
        run(&line, errors)?;
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
