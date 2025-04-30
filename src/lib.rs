#![allow(missing_docs, clippy::missing_docs_in_private_items, dead_code)]

mod doc;

use std::{env, fmt::Debug, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnvVarDef<T> {
    name: String,

    default: Option<T>,

    title: Option<String>,
    description: Option<String>,
    example: Option<T>,
}

impl<T> EnvVarDef<T> {
    pub fn new<S>(name: &S) -> Self
    where S: ToString + ?Sized {
        Self {
            name: name.to_string(),
            title: None,
            description: None,
            default: None,
            example: None,
        }
    }

    #[must_use]
    pub fn with_default(
        mut self,
        default: T,
    ) -> Self {
        self.default = Some(default);
        self
    }

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

    #[must_use]
    pub fn with_example(
        mut self,
        example: T,
    ) -> Self {
        self.example = Some(example);
        self
    }
}

/// Errors which could occure during the `EnvVarDef::load` method
#[derive(thiserror::Error, Clone, Debug, PartialEq, Eq)]
pub enum LoadError<ParseErrorT: Debug> {
    #[error("Cannot load Env Var {0}, either not set or not valid unicode encoded. error: {1:?}")]
    CannotLoad(String, env::VarError),
    #[error("Cannot parse Env Var {0} value {1}, err: {2:?}")]
    CannotParse(String, String, ParseErrorT),
}

impl<T: FromStr, ParseErrorT> EnvVarDef<T>
where
    T: FromStr<Err = ParseErrorT>,
    ParseErrorT: Debug,
{
    /// Tries to loads environment variable.
    ///
    /// # Errors
    /// - `LoadError::CannotLoad` (if `default` is set, returns `default` value instead of
    ///   this error).
    /// - `LoadError::CannotParse` (if `default` is set, returns `default` value instead
    ///   of this error).
    pub fn load(self) -> Result<T, LoadError<ParseErrorT>> {
        env::var(&self.name)
            .map_err(|e| LoadError::CannotLoad(self.name.clone(), e))
            .and_then(|v| {
                v.parse()
                    .map_err(|e| LoadError::CannotParse(self.name, v, e))
            })
            .map_or_else(|e| self.default.ok_or(e), Ok)
    }
}
