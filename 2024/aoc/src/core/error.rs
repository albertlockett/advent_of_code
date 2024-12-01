
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    IO
}

impl From<std::io::Error> for Error {
    fn from (val: std::io::Error) -> Self {
        Self::IO
    }
}