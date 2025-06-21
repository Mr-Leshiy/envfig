//! Implmentation of the derive `EnvVarDef` macro definition.

mod error;

use error::{Error, Result};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Ident, Visibility, parse_macro_input};

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
    let mut fields_load_init = Vec::new();
    for field in data_struct.fields.iter() {
        let field_ident = field.ident.as_ref().ok_or(Error::Other( "It must be a named struct, unit or tuple structs are not allowed".to_string()))?;
        let field_type = &field.ty;
        let env_var_name = quote! { &stringify!(#field_ident).to_uppercase() };
        
        fields_load_init.push(quote! {
            #field_ident: envfig::EnvVarDef::<#field_type>::new( #env_var_name ).load()?,
        });
    }
    Ok(quote! {
            impl #ident {
                #visibility fn load() -> envfig::Result<Self> {
                    Ok(Self {
                        #(#fields_load_init)*
                    })
                }
            }
    })
}
