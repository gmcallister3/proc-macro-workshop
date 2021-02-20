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
        use std::error::Error;
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

        impl #builder_data {
            pub fn executable(&mut self, executable: String) -> &mut Self {
                self.executable = Some(executable);
                self
            }

            pub fn args(&mut self, args: Vec<String>) -> &mut Self {
                self.args = Some(args);
                self
            }

            pub fn env(&mut self, env: Vec<String>) -> &mut Self {
                self.env = Some(env);
                self
            }

            pub fn current_dir(&mut self, current_dir: String) -> &mut Self {
                self.current_dir = Some(current_dir);
                self
            }

            pub fn build(&mut self) -> Result<#object_name, Box<dyn Error>> {
                if (self.executable.is_some() && self.args.is_some() && self.env.is_some() && self.current_dir.is_some()) {
                    return Ok(#object_name {
                        executable: self.executable.as_ref().unwrap().clone(),
                        args: self.args.as_ref().unwrap().clone(),
                        env: self.env.as_ref().unwrap().clone(),
                        current_dir: self.current_dir.as_ref().unwrap().clone()
                    })
                }
                Err("all fields are required".into())
            }
        }
    };
    // eprintln!("TOKENS: {}", expanded);
    TokenStream::from(expanded)
}
