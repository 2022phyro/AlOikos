extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lit};
use heck::{ToPascalCase, ToLowerCamelCase, ToSnakeCase};

#[proc_macro_derive(StrChoice, attributes(strchoice))]
pub fn derive_str_choice(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    // default case
    let mut case_type = "camel".to_string();

    for attr in input.attrs.iter() {
        if attr.path().is_ident("strchoice") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("case") {
                    if let Ok(Lit::Str(litstr)) = meta.value().and_then(|v| v.parse()) {
                        case_type = litstr.value();
                    }
                }
                Ok(())
            });
        }
    }

    let variant_matches = if let syn::Data::Enum(ref data_enum) = input.data {
        data_enum.variants.iter().map(|v| {
            let ident = &v.ident;
            let name = ident.to_string();
            let converted = match case_type.as_str() {
                "snake" => name.to_snake_case(),
                "camel" => name.to_lower_camel_case(),
                "pascal" => name.to_pascal_case(),
                _ => name.clone(),
            };
            quote! { #enum_name::#ident => #converted.to_string(), }
        })
    } else {
        panic!("StrChoice can only be derived for enums");
    };

    let expanded = quote! {
        impl StrChoice for #enum_name {
            fn to_str(&self) -> String {
                match self {
                    #(#variant_matches)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
