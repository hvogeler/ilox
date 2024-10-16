
pub type Errors = Vec<Error>;

#[derive(Debug)]
pub enum Error {
    CodeError {
        line: usize,
        location: Option<String>,
        message: String,
    },
    CompilerError(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::CompilerError(e.to_string())
    }
}