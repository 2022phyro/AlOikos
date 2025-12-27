extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Meta, Lit};
use heck::{CamelCase, SnakeCase, MixedCase};

#[macro_export]
macro_rules! new_model {
    ($model:path, { $($field:ident : $value:expr),* $(,)? }) => {{
        use sea_orm::{ActiveValue::{Set, NotSet}};
        // use sea_orm::Te
        $model {
            id: NotSet,
            created_at: NotSet,
            updated_at: NotSet,
            $($field: Set($value),)*
            ..Default::default()
        }
    }};
}

#[macro_export]
macro_rules! get_or_create {
    ($db:expr, $entity:ident, $id:expr, { $($field:ident : $value:expr),* $(,)? }) => {{
        use sea_orm::{
            EntityTrait, ActiveValue::Set, IntoActiveModel,
        };
        use chrono::Utc;

        match $entity::find_by_id($id.clone()).one($db).await {
            Ok(Some(existing)) => Ok(existing),
            Ok(None) => {
                let new_model = <$entity as EntityTrait>::Model {
                    id: $id.clone(),
                    $(
                        $field: $value.clone(),
                    )*
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    ..Default::default()
                };

                let active_model = new_model.into_active_model();
                match active_model.insert($db).await {
                    Ok(inserted) => Ok(inserted),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }};
}


#[proc_macro_derive(StrChoice, attributes(strchoice))]
pub fn derive_str_choice(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    // Default case
    let mut case_type = "camel".to_string();

    // Check for #[strchoice(case="...")]
    for attr in input.attrs.iter() {
        if attr.path.is_ident("strchoice") {
            if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                for nested in meta_list.nested.iter() {
                    if let syn::NestedMeta::Meta(Meta::NameValue(nv)) = nested {
                        if nv.path.is_ident("case") {
                            if let Lit::Str(litstr) = &nv.lit {
                                case_type = litstr.value();
                            }
                        }
                    }
                }
            }
        }
    }

    // Map variants
    let variant_matches = if let syn::Data::Enum(ref data_enum) = input.data {
        data_enum.variants.iter().map(|v| {
            let ident = &v.ident;
            let name = ident.to_string();
            // convert at compile time using heck
            let converted = match case_type.as_str() {
                "snake" => name.to_snake_case(),
                "camel" => name.to_camel_case(),
                "pascal" => name.to_mixed_case(),
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