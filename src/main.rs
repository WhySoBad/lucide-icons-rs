use std::collections::BTreeMap;

use quote::quote;
use serde::Deserialize;
use syn::LitChar;

mod cli;

fn main() {
    let icon_str = std::fs::read_to_string("info.json").expect("should read font info.json");
    let icons: BTreeMap<String, IconInfo> =
        serde_json::from_str(&icon_str).expect("should deserialize font icon infos");

    let (names, variant_names, unicodes) = icons
        .iter()
        .map(|(key, icon)| {
            let name = syn::Ident::new(
                &key.split('-').map(|part| {
                    let mut chars = part.chars();
                    match chars.next() {
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                        None => String::new(),
                    }
                }).collect::<String>(),
                proc_macro2::Span::call_site(),
            );
            let unicode = syn::Lit::Char(LitChar::new(icon.unicode(), proc_macro2::Span::call_site()));

            (key.clone(), name, unicode)
        })
        .collect::<(Vec<_>, Vec<_>, Vec<_>)>();

    let variants = names.iter().zip(variant_names.iter()).map(|(name, variant)| {
        let doc_msg = format!("[{}](https://lucide.dev/icons/{}) icon", name, name);
        quote! {
            #[doc = #doc_msg]
            #variant
        }
    }).collect::<Vec<_>>();

    let output = quote! {
        #[derive(Debug)]
        pub enum Icon {
            #(#variants),*
        }

        impl Icon {
            /// Unicode character of an icon
            pub fn unicode(&self) -> char {
                match self {
                    #(Self::#variant_names => #unicodes),*
                }
            }

            /// Get an icon from it's name
            ///
            /// The names need to be all-lowercase-dashed (e.g. `app-window`)
            pub fn from_name(icon_name: &str) -> Option<Self> {
                match icon_name {
                    #(#names => Some(Icon::#variant_names)),*,
                    &_ => None
                }
            }
        }

        impl std::fmt::Display for Icon {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let name = match self {
                    #(Self::#variant_names => #names),*
                };
                write!(f, "{name}")
            }
        }
    };

    let str = prettyplease::unparse(&syn::parse2(output).expect("should be valid token stream"));
    std::fs::write("out.rs", &str).expect("should write out.rs")
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
struct IconInfo {
    encoded_code: String,
    prefix: String,
    class_name: String,
    unicode: String,
}

impl IconInfo {
    pub fn unicode(&self) -> char {
        let bytes = u16::from_str_radix(&self.encoded_code.as_str()[1..], 16).expect("should parse icon unicode as u16");
        char::from_u32(bytes as u32).expect("should be a vaild unicode character")
    }
}