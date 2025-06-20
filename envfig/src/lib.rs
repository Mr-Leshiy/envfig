#![doc = include_str!("../../README.md")]

mod doc;
pub mod validator;

use std::{env, fmt::Debug, str::FromStr};

use validator::Validator;

/// Represents the definition of an environment variable, including its name,
/// optional default value, metadata (title, description, example), and optional
/// validation logic.
///
/// `T` is the type of the environment variable's value.
/// `V` is the type of the validator used to check the value's validity.
///
/// # Example
/// ```rust
/// use std::{env, str::FromStr};
///
/// use envfig::{EnvVarDef, LoadError, validator::Validator};
///
/// struct Positive;
///
/// impl Validator<i32> for Positive {
///     type Err = String;
///
///     fn validate(
///         self,
///         val: i32,
///     ) -> Result<i32, Self::Err> {
///         if val > 0 {
///             Ok(val)
///         } else {
///             Err("Value must be positive".into())
///         }
///     }
/// }
///
/// unsafe {
///     env::set_var("APP_PORT", "8080");
/// }
///
/// let port = EnvVarDef::new("APP_PORT")
///     .with_title("Application Port")
///     .with_description("The port the application listens on")
///     .with_example(8080)
///     .with_validator(Positive)
///     .load()
///     .unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnvVarDef<T, V = ()> {
    name: String,

    default: Option<T>,

    title: Option<String>,
    description: Option<String>,
    example: Option<T>,

    validator: Option<V>,
}

impl<T, V> EnvVarDef<T, V> {
    /// Creates a new `EnvVarDef` with the specified environment variable `name`.
    pub fn new<S>(name: &S) -> Self
    where S: ToString + ?Sized {
        Self {
            name: name.to_string(),
            title: None,
            description: None,
            default: None,
            example: None,
            validator: None,
        }
    }

    /// Sets a default value for the environment variable.
    #[must_use]
    pub fn with_default(
        mut self,
        default: T,
    ) -> Self {
        self.default = Some(default);
        self
    }

    /// Sets a `title` for the environment variable.
    #[must_use]
    pub fn with_title<S>(
        mut self,
        title: &S,
    ) -> Self
    where
        S: ToString + ?Sized,
    {
        self.title = Some(title.to_string());
        self
    }

    /// Sets a `description` for the environment variable.
    #[must_use]
    pub fn with_description<S>(
        mut self,
        description: &S,
    ) -> Self
    where
        S: ToString + ?Sized,
    {
        self.description = Some(description.to_string());
        self
    }

    /// Sets an `example` value for the environment variable.
    #[must_use]
    pub fn with_example(
        mut self,
        example: T,
    ) -> Self {
        self.example = Some(example);
        self
    }

    /// Sets a `validator` for the environment variable value.
    #[must_use]
    pub fn with_validator(
        mut self,
        validator: V,
    ) -> Self {
        self.validator = Some(validator);
        self
    }
}

/// Errors which could occure during the `EnvVarDef::load` method
#[derive(thiserror::Error, Clone, Debug, PartialEq, Eq)]
pub enum LoadError<ParseErrorT: Debug, ValidationErrorT> {
    /// The environment variable could not be loaded due to it not being set
    /// or not being valid Unicode.
    #[error("Cannot load Env Var {0}, either not set or not valid unicode encoded. error: {1:?}")]
    CannotLoad(String, env::VarError),

    /// The environment variable was set but could not be parsed into the expected type.
    #[error("Cannot parse Env Var {0} value {1}, err: {2:?}")]
    CannotParse(String, String, ParseErrorT),

    /// The environment variable value did not pass validation.
    #[error(transparent)]
    ValidationError(#[from] ValidationErrorT),
}

impl<T: FromStr, V, ParseErrorT, ValidationErrorT> EnvVarDef<T, V>
where
    T: FromStr<Err = ParseErrorT>,
    V: Validator<T, Err = ValidationErrorT>,
    ParseErrorT: Debug,
{
    /// Tries to loads environment variable.
    ///
    /// # Errors
    /// - `LoadError::CannotLoad` (if `default` is set, returns `default` value instead of
    ///   this error).
    /// - `LoadError::CannotParse` (if `default` is set, returns `default` value instead
    ///   of this error).
    /// - `LoadError::ValidationError`.
    pub fn load(self) -> Result<T, LoadError<ParseErrorT, ValidationErrorT>> {
        env::var(&self.name)
            .map_err(|e| LoadError::CannotLoad(self.name.clone(), e))
            .and_then(|v| {
                v.parse()
                    .map_err(|e| LoadError::CannotParse(self.name, v, e))
            })
            .map_or_else(|e| self.default.ok_or(e), Ok)
            .and_then(|v| {
                if let Some(validator) = self.validator {
                    Ok(validator.validate(v)?)
                } else {
                    Ok(v)
                }
            })
    }
}

impl<T: FromStr, V, ParseErrorT, ValidationErrorT> EnvVarDef<T, V>
where
    T: FromStr<Err = ParseErrorT>,
    V: Validator<Option<T>, Err = ValidationErrorT>,
    ParseErrorT: Debug,
{
    /// Tries to loads environment variable that is optional.
    /// Does not fails on `LoadError::CannotLoad` and `LoadError::CannotParse`,
    /// instead ignores `default` value and returns `None`.
    ///
    /// # Errors
    /// - `LoadError::ValidationError`.
    pub fn load_option(self) -> Result<Option<T>, LoadError<ParseErrorT, ValidationErrorT>> {
        let val = env::var(&self.name).ok().and_then(|v| v.parse::<T>().ok());
        if let Some(validator) = self.validator {
            Ok(validator.validate(val)?)
        } else {
            Ok(val)
        }
    }
}
