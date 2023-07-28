#![recursion_limit = "256"]

use proc_macro::TokenStream;

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use syn::Ident; 
use syn::Ty;

#[proc_macro_derive(Table)]
pub fn table(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_table(&ast);
    gen.parse().unwrap()
}

fn impl_table(ast: &syn::DeriveInput) -> quote::Tokens {
    let struct_name = &ast.ident;
    let scaffold_name = Ident::new(format!("_{}_SCAFFOLD", struct_name.to_string().to_ascii_uppercase()));

    if let syn::Body::Struct(body) = &ast.body {
        let fields = body.fields();
        let mut scaffold_fields: Vec<String> = Vec::new();
        scaffold_fields.reserve(fields.len());
        let mut field_str_names: Vec<String> = Vec::new();
        field_str_names.reserve(fields.len());
        let mut field_names: Vec<String> = Vec::new();
        field_names.reserve(fields.len());

        for field in fields {
            let sqlite_type = {
                let field_type = match &field.ty {
                    Ty::Path(_, path) => path
                        .segments
                        .clone()
                        .iter()
                        .map(|x| x.ident.to_string() )
                        .collect::<Vec<String>>(),
                    _=> panic!("Invalid field category"), 
                };
            
                match field_type[0].as_str() {
                    "u64" => "SqliteType::Integer",
                    "i64" => "SqliteType::Integer",
                    "f64" => "SqliteType::Real",
                    "String" => "SqliteType::Text",
                    _ => panic!("Invalid field type")
                }

            };

            let ident = field.ident.as_ref().unwrap().to_string();

            scaffold_fields.push(format!("(\"{}\", {})", ident, sqlite_type));
            field_str_names.push(format!("\"{}\"", ident));
            field_names.push(format!("{}", ident));
        }

        quote! {
                /// Not intended for direct use. See [Table] trait.
                static #scaffold_name: once_cell::sync::Lazy<std::sync::Arc<std::collections::HashMap<String, SqliteType>>> = Lazy::new(|| {
                    let map: std::collections::HashMap<String, SqliteType> = [
                        #(#scaffold_fields),*
                    ];

                    std::sync::Arc::new(map)
                });
                
                impl Table for #struct_name {
                    fn scaffold<T>() -> std::sync::Arc<std::collections::HashMap<String, SqliteType>> {
                        #scaffold_name.clone()
                    }

                    fn get<T>(&self, col: &str) -> anyhow::Result<&T> {
                        let out: Box<dyn std::any::Any>;

                        #(
                            if col == #field_str_names {
                                out = Box::new(&self.#field_names);
                                return out
                                    .downcast_ref::<T>()
                                    .ok_or(anyhow::anyhow!("Invalid type!"));
                            }
                        )*

                        return anyhow::Result::Err(anyhow::anyhow!("No such column!"));
                    }
                }
            }
    } else {
       panic!("#[derive(Table)] is only defined for structs, not for enums!");
    }
}
