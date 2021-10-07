use crate::daemon::{client::RevaultDError, config::ConfigError};
use std::convert::From;
use std::io::ErrorKind;

#[derive(Debug, Clone)]
pub enum Error {
    ConfigError(ConfigError),
    RevaultDError(RevaultDError),
    UnexpectedError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ConfigError(e) => write!(f, "{}", e),
            Self::RevaultDError(e) => match e {
                RevaultDError::Unexpected(e) => write!(f, "{}", e),
                RevaultDError::NoAnswer => write!(f, "Daemon did not answer"),
                RevaultDError::Transport(Some(ErrorKind::ConnectionRefused), _) => {
                    write!(f, "Failed to connect to daemon")
                }
                RevaultDError::Transport(kind, e) => {
                    if let Some(k) = kind {
                        write!(f, "{} [{:?}]", e, k)
                    } else {
                        write!(f, "{}", e)
                    }
                }
                RevaultDError::Rpc(code, e) => {
                    write!(f, "[{:?}] {}", code, e)
                }
            },
            Self::UnexpectedError(e) => write!(f, "Unexpected error: {}", e),
        }
    }
}

impl From<ConfigError> for Error {
    fn from(error: ConfigError) -> Self {
        Error::ConfigError(error)
    }
}

impl From<RevaultDError> for Error {
    fn from(error: RevaultDError) -> Self {
        Error::RevaultDError(error)
    }
}
