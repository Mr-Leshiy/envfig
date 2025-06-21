//! Implmentation of the derive `EnvVarDef` macro definition.

mod error;

use error::{Error, Result};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, Ident, Visibility, parse_macro_input};

/// TODO
#[proc_macro_derive(EnvVarDef, attributes(envardef))]
pub fn derive_envvardef(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    generate(input).unwrap_or_else(Into::into).into()
}

fn generate(input: DeriveInput) -> Result<TokenStream> {
    match &input.data {
        Data::Struct(data_struct) => process_struct(&input.vis, &input.ident, &data_struct),
        _ => Err(Error::Other("It must be a struct only".to_string())),
    }
}

fn process_struct(
    visibility: &Visibility,
    ident: &Ident,
    data_struct: &DataStruct,
) -> Result<TokenStream> {
    let Fields::Named(_fields) = &data_struct.fields else {
        Err(Error::Other(
            "It must be a named struct, unit or tuple structs are not allowed".to_string(),
        ))?
    };

    Ok(quote! {
            impl #ident {
                #visibility fn load() -> envfig::Result<()> {
                    Ok(())
                }
            }
    })
}
