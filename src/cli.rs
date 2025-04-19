use std::fmt::Display;

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// Directory to write the output library to
    #[arg(long, short, default_value_t = String::from("out"))]
    pub output: String,

    /// Version of the iced crate to use in the output library
    #[arg(long, short, default_value_t = String::from("0.13"))]
    pub iced_version: String,

    /// Name of the output library
    #[arg(long, short, default_value_t = String::from("lucide-icons"))]
    pub name: String,

    /// Description of the output library
    #[arg(long, short, default_value_t = String::from("Rust definitions for lucide-icons"))]
    pub description: String,

    /// Rust edition of the output library
    #[arg(long, short, value_enum, default_value_t = Edition::Year2024)]
    pub edition: Edition,

    /// License of the output library
    #[arg(long, short, default_value_t = String::from("MIT"))]
    pub license: String,

    /// Categories of the output library
    #[arg(long, short, value_delimiter = ',', value_parser, default_values_t = vec!["gui"].into_iter().map(String::from).collect::<Vec<_>>())]
    pub categories: Vec<String>,

    /// Keywords of the output library
    #[arg(long, short, value_delimiter = ',', value_parser, default_values_t = vec!["lucide-icons", "lucide", "icon", "iced", "font"].into_iter().map(String::from).collect::<Vec<_>>())]
    pub keywords: Vec<String>,

    /// Homepage url of the output library
    #[arg(long, short = 'w')]
    pub homepage_url: Option<String>,

    /// Repository url of the output library
    #[arg(long, short)]
    pub repository_url: Option<String>,

    /// Path to the README of the output library
    #[arg(long, short, default_value_t = String::from("README.md"))]
    pub readme_path: String,

    /// Authors of the output library
    #[arg(long, short, value_delimiter = ',', value_parser, default_values_t = Vec::<String>::new())]
    pub authors: Vec<String>,

    /// Tag of the lucide icons release
    pub tag: String,
}

#[derive(clap::ValueEnum, Clone)]
pub enum Edition {
    #[clap(name = "2015")]
    Year2015,
    #[clap(name = "2018")]
    Year2018,
    #[clap(name = "2021")]
    Year2021,
    #[clap(name = "2024")]
    Year2024,
}

impl Display for Edition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match &self {
            Edition::Year2015 => "2015",
            Edition::Year2018 => "2018",
            Edition::Year2021 => "2021",
            Edition::Year2024 => "2024",
        })
    }
}
