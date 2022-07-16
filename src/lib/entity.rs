use std::fmt;

pub enum CalcEntityError {
    OperationError,
    NoDefinitionError,
}

pub trait CalcEntity<T = Self>: fmt::Debug + std::marker::Sized {
    fn add(&self, _other: &T) -> Result<Self, CalcEntityError> {
        Err(CalcEntityError::NoDefinitionError)
    }
}

impl CalcEntity for f64 {
    fn add(&self, _other: &f64) -> Result<f64, CalcEntityError> {
        Ok(self + _other)
    }
}

impl CalcEntity<f64> for &[f64] {}
