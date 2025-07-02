use std::collections::BTreeMap;

use anyhow::Context;
use quote::quote;
use syn::LitChar;

use crate::{cli::Cli, info::IconInfo};

pub fn generate_readme(name: &str, version: &str) -> String {
    let lib_name = name.replace('-', "_");
    format!(
r#####"

# {name}

Auto-generated rust icon definitions for [lucide icons](https://github.com/lucide-icons/lucide) [version {version}](https://github.com/lucide-icons/lucide/releases/tag/{version})

The library provides an `Icon` enum which contains all lucide icon variants:

```rust
use {lib_name}::Icon;

fn main() {{
    let icon = Icon::Anvil;
    assert_eq!(format!("{{icon}}"), String::from("anvil"));
    println!("unicode = {{}}", icon.unicode());
}}
```

With the `iced` feature the library also provides the icons as iced widgets:

```rust
use {lib_name}::lucide_font_bytes;
use {lib_name}::iced::icon_anvil;

fn setup_application() {{
    // get font bytes for the bundled font
    let bytes = lucide_font_bytes();

    // add the font to iced
    let settings = iced::Settings {{
        fonts: vec![bytes.into()],
        ..Default::default()
    }};

    // run app with settings...
}}

fn view() -> iced::Element<'_, Message, Theme, iced::Renderer> {{
    iced::widget::column![
        icon_anvil()
    ].into()
}}

```

For more details have a look at the [generator repository page](https://github.com/WhySoBad/lucide-icons-rs/)

"#####)
    .trim().to_string()
}

fn vec_to_str(vec: &[String]) -> String {
    let vec_str = vec
        .iter()
        .map(str_with_parens)
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{vec_str}]")
}

fn str_with_parens(str: &String) -> String {
    format!(r#""{str}""#)
}

pub fn generate_cargo_toml(cli: &Cli) -> String {
    let fields = vec![
        ("name", Some(str_with_parens(&cli.name))),
        ("description", Some(str_with_parens(&cli.description))),
        ("version", Some(str_with_parens(&cli.tag))),
        ("edition", Some(str_with_parens(&cli.edition.to_string()))),
        ("license", Some(str_with_parens(&cli.license))),
        ("authors", Some(vec_to_str(&cli.authors))),
        ("categories", Some(vec_to_str(&cli.categories))),
        ("keywords", Some(vec_to_str(&cli.keywords))),
        ("homepage", cli.homepage_url.as_ref().map(str_with_parens)),
        (
            "repository",
            cli.repository_url.as_ref().map(str_with_parens),
        ),
        ("readme", Some(str_with_parens(&cli.readme_path))),
    ];

    let package_str = fields
        .iter()
        .filter_map(|(key, value)| value.as_ref().map(|val| format!("{key} = {val}")))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r##"
[package]
{package_str}

[features]
default = []
iced = ['dep:iced']

[dependencies]
iced = {{ version = '{}', optional = true, features = ["advanced"], default-features = false }}
"##,
        cli.iced_version
    )
    .trim()
    .to_string()
}

pub fn generate_library() -> anyhow::Result<String> {
    let output = quote! {
        #[cfg(feature = "iced")]
        pub mod iced;
        mod icon;
        pub use crate::icon::Icon;

        /// Bytes of the lucide font
        ///
        /// Always use this font when relying on the icons of this crate as it may be
        /// that the system installation of the font has a different version than the
        /// one used by this crate
        pub fn lucide_font_bytes() -> &'static [u8] {
            include_bytes!("../lucide.ttf")
        }
    };

    let file_str =
        prettyplease::unparse(&syn::parse2(output).context("Output should be valid TokenStream")?);
    Ok(file_str)
}

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
        #[derive(Debug, Clone, Copy)]
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

    let file_str =
        prettyplease::unparse(&syn::parse2(output).context("Output should be valid TokenStream")?);

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
                    iced::widget::text(#unicode.to_string()).font(iced::Font::with_name("lucide"))
                }
            }
        })
        .collect::<Vec<_>>();

    let output = quote! {
        // use iced::widget::text;

        // fn base_icon<'a>(icon: char) -> iced::widget::Text<'a> {
        //     text(icon.to_string()).font(iced::Font::with_name("lucide"))
        // }

        #(#functions)*
    };

    let file_str =
        prettyplease::unparse(&syn::parse2(output).context("Output should be valid TokenStream")?);

    Ok(file_str)
}
