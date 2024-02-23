use crate::error::DomainError;

pub type DomainResult<T, E = DomainError> = anyhow::Result<T, E>;
