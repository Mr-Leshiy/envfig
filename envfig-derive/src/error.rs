//! Error definition

use proc_macro2::TokenStream;
use quote::quote;

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("{0}")]
    Syn(#[from] syn::Error),

    #[error("{0}")]
    #[allow(dead_code)]
    Other(String),
}

impl From<Error> for TokenStream {
    fn from(err: Error) -> Self {
        match err {
            Error::Syn(err) => err.into_compile_error(),
            Error::Other(err) => quote!(::core::compile_error!(#err)),
        }
    }
}

/// A convienient type alias for `Result<T, Error>`.
pub(crate) type Result<T> = std::result::Result<T, Error>;
