//! Module contains a `Validator` trait definition along with the some implementation's of
//! this trait.

mod min_max;

pub use min_max::SatMinMaxValidator;

/// An environment variable validator, after the variable has been loaded and parsed.
/// Also important to note that it also validates the default value.
pub trait Validator<T> {
    /// Validate function of the provided environment variable value.
    /// Allows to either return an error or re-assign value to something else.
    fn validate(
        self,
        val: T,
    ) -> anyhow::Result<T>;

    /// A human description of the `Validatar` instance which would be included into the
    /// `EnvVarDef::doc`
    fn description(&self) -> Option<String> {
        None
    }
}

impl<T> Validator<T> for () {
    fn validate(
        self,
        val: T,
    ) -> anyhow::Result<T> {
        Ok(val)
    }
}
