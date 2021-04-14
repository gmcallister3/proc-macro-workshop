extern crate proc_macro;

use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Data, Expr, Fields};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syn_tree = parse_macro_input!(input as DeriveInput);
    let exp_tree = parse_macro_input!(input as Expr);
    let object_name = syn_tree.ident;
    let builder_name = format_ident!("{}Builder", object_name);
    // println!("tree {}", syn_tree.data);
    // let fields = parse_macro_input!(input as FieldsNamed).named;

    //dynamically generate builder data based on input fields
    //dynamically generate build() to handle either Option<> or non-option fields

    // let test_builder = match exp_tree {
    //     Expr::Path(path) => {
    //         path.path.segments
    //     }
    //     _ => {}
    // }

    let builder_code = match syn_tree.data {
        Data::Struct(data) => {
            match data.fields {
                Fields::Named(fields) => {
                    let field_tokens = fields.named.iter().map(|field| {
                        let field_name = field.ident.unwrap();
                        return quote! {
                            #field_name: Option<#field.ty>,
                        }
                    });

                    quote! {
                        pub struct #builder_name {
                            #(#field_tokens)*
                        }
                    }
                }
                _ => unimplemented!("Doesn't support unnamed fields.")
            }
        }
        _ => unimplemented!("Builder macro on non-struct code.")
    };

    let expanded = quote! {
        use std::error::Error;
        impl #object_name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None
                }
            }
        }

        #builder_code

        impl #builder_name {
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
    eprintln!("TOKENS: {}", expanded);
    expanded.into()
}
