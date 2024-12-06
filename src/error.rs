use std::error::Error;
use thiserror::Error;

pub type Result<T, E = Report> = color_eyre::Result<T, E>;
pub struct Report(color_eyre::Report);

impl std::fmt::Debug for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<E> From<E> for Report
where
    E: Into<color_eyre::Report>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum AppError {
    #[error("Data File Open Error: {0}")]
    DataOpen(String),

    #[error("Data File Parsing Error: {0}")]
    DataParse(String),

    #[error("Error during execution: {0}")]
    LogicalError(String),

    #[error("Internal error: {0}")]
    Other(String),
}

impl From<Box<dyn Error + Send + Sync>> for AppError {
    fn from(value: Box<dyn Error + Send + Sync>) -> Self {
        Self::Other(value.to_string())
    }
}
