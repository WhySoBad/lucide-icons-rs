# lucide-icons-rs

> Icon definition generator for [lucide icon](https://github.com/lucide-icons/lucide) releases

This project is designed to run in an interval and automatically generate a rust library for every new lucide icons release.

## CLI

The `lucide-icons` cli supports the following arguments

| Argument         | Description                                      | Default value                                        |
| ---------------- | ------------------------------------------------ | ---------------------------------------------------- |
| `output`         | Directory where the library should be written to | `out`                                                |
| `name`           | Name of the output library                       | `lucide-icons`                                       |
| `description`    | Description of the output library                | `rust definitions for lucide-icons`                  |
| `edition`        | Rust edition of the output library               | `2024`                                               |
| `categories`     | Categories of the output library                 | `["gui"]`                                            |
| `keywords`       | Keywords of the output library                   | `["lucide-icons", "lucide", "icon", "iced", "font"]` |
| `homepage-url`   | Url to the output library homepage               | _none_                                               |
| `repository-url` | Url to the output library repository             | _none_                                               |
| `readme-path`    | Path to the README of the output library         | `README.md`                                          |
| `autors`         | Authors which worked on the output library       | `[]`                                                 |

### Usage

To generate the icon definitions as a library for a specific release tag one can run the following command

```bash
lucide-icons [arguments] <tag>
```

where `tag` is the name of a release tag of the [lucide-icons repository](https://github.com/lucide-icons/lucide)

### Using docker

The cli is also available in the `ghcr.io/whysobad/lucide-icons` docker image.

To generate a new library, the following command can be used:

```bash
docker run -v ./out:/app/out ghcr.io/whysobad/lucide-icons lucide-icons [arguments] <tag>
```

## Generated library

The generated library contains the `Icon` rust enum which holds all lucide icons available as variants:

```rust
use lucide_icons::Icon;

let icon = Icon::Anvil;
// the variants implement `Display` which returns their icon name
assert_eq!(icon.to_string(), String::from("anvil"));
// using the `unicode` method one can get the unicode character for every variant
println!("unicode = {}", icon.unicode());
```

Additionally, the library also provides an accessor for the bundled lucide icons font:

```rust
use lucide_icons::lucide_font_bytes;

// get font bytes for the bundled font
let font_bytes = lucide_font_bytes();
```

The library also has the optional `iced` (or `iced-cosmic`) feature which also provides lucide icons as pre-defined iced widgets:

```rust
use lucide_icons::iced::icon_anvil;

// add the font to iced
let settings = iced::Settings { fonts: vec![font_bytes.into()], ..Default::default() };

fn view() -> iced::Element<'_, Message, Theme, iced::Renderer> {
    iced::widget::column![
        icon_anvil()
    ].into()
}
```