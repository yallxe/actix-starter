#[derive(Debug)]
pub enum DomainError {
    InternalError(anyhow::Error),
}

impl<E> From<E> for DomainError
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(error: E) -> Self {
        DomainError::InternalError(anyhow::Error::new(error))
    }
}