extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let _ = input;
    let object_name = parse_macro_input!(input as DeriveInput).ident;
    let builder_data = format_ident!("{}Builder", object_name);
    // let fields = parse_macro_input!(input as FieldsNamed).named;

    let expanded = quote! {
        impl #object_name {
            pub fn builder() -> #builder_data {
                #builder_data {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None
                }
            }
        }

        pub struct #builder_data {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>
        }
    };
    TokenStream::from(expanded)
}
