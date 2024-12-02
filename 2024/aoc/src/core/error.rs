pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    Arrow,
    Datafusion,
    IO,
}

impl From<std::io::Error> for Error {
    fn from(_val: std::io::Error) -> Self {
        Self::IO
    }
}

impl From<arrow::error::ArrowError> for Error {
    fn from(_val: arrow::error::ArrowError) -> Self {
        Self::Arrow
    }
}

impl From<datafusion::error::DataFusionError> for Error {
    fn from(_val: datafusion::error::DataFusionError) -> Self {
        Self::Datafusion
    }
}
