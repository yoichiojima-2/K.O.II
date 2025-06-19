use std::fmt;
use std::error::Error;
use std::io;

#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    Audio(String),
    Config(String),
    Command(String),
    Sample(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "I/O error: {}", err),
            AppError::Audio(msg) => write!(f, "Audio error: {}", msg),
            AppError::Config(msg) => write!(f, "Configuration error: {}", msg),
            AppError::Command(msg) => write!(f, "Command error: {}", msg),
            AppError::Sample(msg) => write!(f, "Sample error: {}", msg),
        }
    }
}

impl Error for AppError {}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AppError::Audio("Failed to initialize audio".to_string());
        assert_eq!(err.to_string(), "Audio error: Failed to initialize audio");
        
        let err = AppError::Command("Invalid command".to_string());
        assert_eq!(err.to_string(), "Command error: Invalid command");
        
        let err = AppError::Config("Missing configuration".to_string());
        assert_eq!(err.to_string(), "Configuration error: Missing configuration");
        
        let err = AppError::Sample("Sample not found".to_string());
        assert_eq!(err.to_string(), "Sample error: Sample not found");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let app_err: AppError = io_err.into();
        
        match app_err {
            AppError::Io(_) => assert!(true),
            _ => assert!(false, "Expected Io variant"),
        }
    }
}