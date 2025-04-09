use std::collections::BTreeMap;

use anyhow::Context;
use quote::quote;
use syn::LitChar;

use crate::IconInfo;

pub fn generate_icons_enum(icons: &BTreeMap<String, IconInfo>) -> anyhow::Result<String> {
    let (names, variant_names, unicodes) = icons
        .iter()
        .map(|(key, icon)| {
            let name = syn::Ident::new(
                &key.split('-')
                    .map(|part| {
                        let mut chars = part.chars();
                        match chars.next() {
                            Some(first) => {
                                first.to_uppercase().collect::<String>() + chars.as_str()
                            }
                            None => String::new(),
                        }
                    })
                    .collect::<String>(),
                proc_macro2::Span::call_site(),
            );
            let unicode =
                syn::Lit::Char(LitChar::new(icon.unicode(), proc_macro2::Span::call_site()));

            (key.clone(), name, unicode)
        })
        .collect::<(Vec<_>, Vec<_>, Vec<_>)>();

    let variants = names
        .iter()
        .zip(variant_names.iter())
        .map(|(name, variant)| {
            let doc_msg = format!("[{}](https://lucide.dev/icons/{}) icon", name, name);
            quote! {
                #[doc = #doc_msg]
                #variant
            }
        })
        .collect::<Vec<_>>();

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

    let file_str = prettyplease::unparse(&syn::parse2(output).context("Output should be valid TokenStream")?);

    Ok(file_str)
}

pub fn generate_iced_icons(icons: &BTreeMap<String, IconInfo>) -> anyhow::Result<String> {
    let functions = icons
        .iter()
        .map(|(key, icon)| {
            let name = syn::Ident::new(
                &(String::from("icon_").to_owned() + key.replace('-', "_").as_str()),
                proc_macro2::Span::call_site(),
            );
            let unicode =
                syn::Lit::Char(LitChar::new(icon.unicode(), proc_macro2::Span::call_site()));

            let doc_msg = format!("[{}](https://lucide.dev/icons/{}) icon", key, key);

            quote! {
                #[doc = #doc_msg]
                pub fn #name<'a>() -> iced::widget::Text<'a> {
                    base_icon(#unicode)
                }
            }
        })
        .collect::<Vec<_>>();

    let output = quote! {
        use iced::widget::text;

        fn base_icon<'a>(icon: char) -> iced::widget::Text<'a> {
            text(icon.to_string())
        }

        #(#functions)*
    };

    let file_str = prettyplease::unparse(&syn::parse2(output).context("Output should be valid TokenStream")?);

    Ok(file_str)
}