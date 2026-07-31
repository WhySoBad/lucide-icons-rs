use std::collections::BTreeMap;

use anyhow::Context;
use quote::quote;
use syn::{LitChar, LitStr};

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
    println!("unicode = {{}}", char::from(icon));
}}
```

## Iced

With the `iced` feature, the library also provides the icons as iced widgets:

```rust
use {lib_name}::LUCIDE_FONT_BYTES;
use {lib_name}::iced::icon_anvil;

fn setup_application() {{
    let settings = iced::Settings {{
        // add bundled font to iced
        fonts: vec![LUCIDE_FONT_BYTES.into()],
        ..Default::default()
    }};

    // run app with settings...
}}

fn view() -> iced::Element<'_, Message, Theme, iced::Renderer> {{
    iced::widget::column![
        // named widget function per icon
        icon_anvil(),
        // widget function per variant
        Icon::Anvil.into()
    ].into()
}}
```

## Serde

With the `serde` feature, the library also provides serialization/deserialization for the `Icon` enum by an icon name.

```rust
use {lib_name}::Icon;

#[derive(serde::Serialize)]
struct IconWrapper {{
    icon: Icon
}}

fn to_json() {{
    let icon = IconWrapper {{ icon: Icon::ArrowUpNarrowWide }};
    let str = serde_json::to_string(&icon).expect("should serialize");
    assert_eq!(str, "{{\"icon\":\"arrow-up-narrow-wide\"}}");
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
serde = ['dep:serde']

[dependencies]
iced = {{ version = '{}', optional = true, features = ["advanced"], default-features = false }}
serde = {{ version = '1.0', optional = true, features = ["derive"] }}
"##,
        cli.iced_version
    )
    .trim()
    .to_string()
}

pub fn generate_library(name: &str, version: &str) -> anyhow::Result<String> {
    let lib_name = name.replace('-', "_");
    let output = quote! {
        #![doc = concat!("Auto-generated rust icon definitions for [lucide icons](https://lucide.dev) [version ", #version, "](https://github.com/lucide-icons/lucide/releases/tag/", #version, ")")]
        //!
        //! This library provides an [`Icon`] enum which contains all lucide icon variants:
        //!
        //! ```rust
        #![doc = concat!("use ", #lib_name, "::Icon;")]
        //!
        //! fn main() {
        //!     let icon = Icon::Anvil;
        //!     assert_eq!(format!("{icon}"), String::from("anvil"));
        //!     println!("unicode = {}", char::from(icon));
        //! }
        //! ```
        //!
        //! Additionally, the underlying lucide ttf bytes are available through the [`LUCIDE_FONT_BYTES`] constant.
        //!
        //! ## Iced
        //!
        //! With the `iced` feature, [iced](https://iced.rs) compatibility can be enabled. This means, [`Icon`] variants can directly be converted to iced widgets
        //! and for every lucide icon variant there will be a named function which returns an icon widget.
        //!
        //! ```rust
        #![doc = concat!("use ", #lib_name, "::{LUCIDE_FONT_BYTES, iced::icon_anvil};")]
        //!
        //! fn setup_application() {
        //!     let settings = iced::Settings {
        //!         // add bundled font to iced
        //!         fonts: vec![LUCIDE_FONT_BYTES.into()],
        //!         ..Default::default()
        //!     };
        //!
        //!     // run app with settings...
        //! }
        //!
        //! fn view() -> iced::Element<'_, Message, Theme, iced::Renderer> {
        //!     iced::widget::column![
        //!         // named widget function per icon
        //!         icon_anvil(),
        //!         // widget function per variant
        //!         Icon::Anvil.into()
        //!     ].into()
        //! }
        //! ```
        //!
        //! ## Serde
        //! With the `serde` feature, the library also provides serialization/deserialization for the [`Icon`] enum by an icon name.
        //!
        //! ```rust
        #![doc = concat!("use ", #lib_name, "::Icon;")]
        //!
        //! #[derive(serde::Serialize)]
        //! struct IconWrapper {
        //!     icon: Icon
        //! }
        //!
        //! fn to_json() {
        //!     let icon = IconWrapper { icon: Icon::ArrowUpNarrowWide };
        //!     let str = serde_json::to_string(&icon).expect("should serialize");
        //!     assert_eq!(str, "{\"icon\":\"arrow-up-narrow-wide\"}");
        //! }
        //! ```
        //!

        #[cfg(feature = "iced")]
        pub mod iced;
        mod icon;
        pub use crate::icon::Icon;

        /// Bytes of the lucide font
        ///
        /// Always use this font when relying on the icons of this crate as it may be
        /// that the system installation of the font has a different version than the
        /// one used by this crate
        pub const LUCIDE_FONT_BYTES: &[u8] = include_bytes!("../lucide.ttf");
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

        /// Representation of a lucide icon
        ///
        /// # Usage
        /// ```rust
        /// let icon = Icon::Anvil;
        ///
        /// // get icon unicode character
        /// let unicode = char::from(icon);
        /// // or by using `unicode` method explicitly
        /// let unicode = Icon::Anvil.unicode();
        ///
        /// // get icon by name
        /// Icon::try_from("anvil").expect("should be valid icon variant");
        ///
        /// // get icon by unicode
        /// Icon::try_from('\u{e1ad}').expect("should be valid icon variant");
        ///
        /// // turn icon into iced text widget
        /// iced::widget::Text::from(Icon::Anvil);
        /// ```
        /// **Important**: All iced-related functionalities require the `iced` feature to be enabled
        ///                which is disabled by default
        #[derive(Debug, Clone, Copy)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
        pub enum Icon {
            #(#variants),*
        }

        impl Icon {
            /// Unicode code point of the icon variant
            ///
            /// **Note**: This is the same as `char::from(icon)`
            pub fn unicode(self) -> char {
                self.into()
            }

            /// Iced icon widget of the icon variant
            ///
            /// **Note**: This is the same as `iced::widget::Text::from(icon)`
            #[cfg(feature = "iced")]
            pub fn widget<'a, Theme>(self) -> iced::widget::Text<'a, Theme>
            where
                Theme: iced::widget::text::Catalog + 'a
            {
                self.into()
            }
        }

        impl TryFrom<&str> for Icon {
            type Error = String;

            fn try_from(icon_str: &str) -> Result<Icon, Self::Error> {
                match icon_str {
                    #(#names => Ok(Icon::#variant_names)),*,
                    &_ => Err(format!("icon '{icon_str}' is not a valid icon variant"))
                }
            }
        }

        impl TryFrom<char> for Icon {
            type Error = String;

            fn try_from(unicode: char) -> Result<Icon, Self::Error> {
                match unicode {
                    #(#unicodes => Ok(Icon::#variant_names)),*,
                    _ => Err(format!("unicode '{unicode}' is not a valid icon unicode"))
                }
            }
        }

        impl From<Icon> for char {
            fn from(icon: Icon) -> char {
                match icon {
                    #(Icon::#variant_names => #unicodes),*
                }
            }
        }

        #[cfg(feature = "iced")]
        impl<'a, Theme> From<Icon> for iced::widget::Text<'a, Theme>
        where
            Theme: iced::widget::text::Catalog + 'a
        {
            fn from(icon: Icon) -> iced::widget::Text<'a, Theme> {
                iced::widget::text(char::from(icon).to_string()).font(iced::Font::with_name("lucide"))
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

            let unicode_str = syn::Lit::Str(LitStr::new(
                icon.unicode().to_string().as_str(),
                proc_macro2::Span::call_site(),
            ));

            let doc_msg = format!("[{}](https://lucide.dev/icons/{}) icon", key, key);

            quote! {
                #[doc = #doc_msg]
                pub fn #name<'a, Theme>() -> iced::widget::Text<'a, Theme>
                where
                    Theme: iced::widget::text::Catalog + 'a
                {
                    iced::widget::text(#unicode_str).font(iced::Font::with_name("lucide"))
                }
            }
        })
        .collect::<Vec<_>>();

    let output = quote! {
        #(#functions)*
    };

    let file_str =
        prettyplease::unparse(&syn::parse2(output).context("Output should be valid TokenStream")?);

    Ok(file_str)
}
