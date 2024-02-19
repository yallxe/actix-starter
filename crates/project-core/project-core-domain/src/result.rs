use crate::error::DomainError;

pub type DomainResult<T, E = DomainError> = Result<T, E>;
