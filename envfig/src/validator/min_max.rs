//! `MinMaxValidator` implementation

use super::Validator;

/// A saturating upper limit and lower limit boundaries validator implementation,
/// for any specified type which implements `PartialOrd` trait.
/// If it exceeds limits during validation returns the value of the limit boundary, never
/// returns the error.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SatMinMaxValidator<T> {
    /// Lower limit boundary
    pub min: Option<T>,
    /// Upper limit boundary
    pub max: Option<T>,
}

impl<T> Validator<T> for SatMinMaxValidator<T>
where T: PartialOrd
{
    type Err = ();

    fn validate(
        self,
        val: T,
    ) -> Result<T, Self::Err> {
        match self.min {
            Some(min) if val < min => return Ok(min),
            _ => {},
        }
        match self.max {
            Some(max) if val > max => return Ok(max),
            _ => {},
        }
        Ok(val)
    }
}
