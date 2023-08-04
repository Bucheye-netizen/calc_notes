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
    let global_name = Ident::new(format!("_{}_FIELDS", struct_name.to_string().to_ascii_uppercase()));
    let table_name = format!("{}Table", struct_name);

    if let syn::Body::Struct(body) = &ast.body {
        let fields = body.fields();
        let mut field_types: Vec<Ident> = Vec::new();
        field_types.reserve(fields.len());
        let mut field_names: Vec<String> = Vec::new();
        field_names.reserve(fields.len());

        for field in fields {
            let sqlite_type = {
                // Slightly inefficient since I'm unnecessarily allocating 
                // a new field_type string. 
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
                    "u64" => Ident::new("crate::model::SqliteType::Integer"),
                    "i64" => Ident::new("crate::model::SqliteType::Integer"),
                    "f64" => Ident::new("crate::model::SqliteType::Real"),
                    "String" => Ident::new("crate::model::SqliteType::Text"),
                    _ => panic!("Invalid field type")
                }

            };

            field_types.push(sqlite_type);
            let ident = field.ident.as_ref().unwrap();
            field_names.push(ident.to_string());
        }

        quote! {
                /// Not intended for direct use. See [Table] trait.
                static #global_name: once_cell::sync::Lazy<std::sync::Arc<std::collections::HashMap<String, SqliteType>>> = Lazy::new(|| {
                    let map: std::collections::HashMap<String, SqliteType> = [
                        #((#field_names.to_string(), #field_types)),*
                    ]
                    .into_iter()
                    .collect::<std::collections::HashMap<String, SqliteType>>();

                    std::sync::Arc::new(map)
                });
                
                impl Table for #struct_name {
                    fn fields() -> std::sync::Arc<std::collections::HashMap<String, SqliteType>> {
                        #global_name.clone()
                    }
                
                    fn name() -> &'static str {
                        return #table_name;
                    }
                }
            }
    } else {
       panic!("#[derive(Table)] is only defined for structs, not for enums!");
    }
}
