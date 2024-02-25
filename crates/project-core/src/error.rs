use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Username is already taken: {username}")]
    UsernameIsTaken {
        username: String,
    },
    #[error("Email is already registered: {email}")]
    EmailAlreadyRegistered {
        email: String,
    },
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    ClientError(#[from] ClientError),
    
    // This is basically an "anything else" variant.
    // If this pops up, the developer probably screwed up.
    #[error(transparent)]
    UnhandledError(#[from] anyhow::Error),
}

// Below are the implementations for "anything else" variant.

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::UnhandledError(err.into())
    }
}