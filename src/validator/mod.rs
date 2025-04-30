//! Module contains a `Validator` trait definition along with the some implementation's of
//! this trait.

use std::fmt::Debug;

/// An environment variable validator, after the variable has been loaded and parsed.
/// Also important to note that it also validates the default value.
pub trait Validator<T> {
    /// The associated error which can be returned from validating.
    type Err: Debug;

    /// Validate function of the provided environment variable value.
    /// Allows to either return an error or re-assign value to something else.
    ///
    /// # Errors
    /// - `Self::Err`
    fn validate(
        &self,
        val: T,
    ) -> Result<T, Self::Err>;
}

impl<T> Validator<T> for () {
    type Err = ();

    fn validate(
        &self,
        val: T,
    ) -> Result<T, Self::Err> {
        Ok(val)
    }
}
