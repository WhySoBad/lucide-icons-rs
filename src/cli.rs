use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// Directory to write the output library to
    #[arg(long, short, default_value_t = String::from("out"))]
    pub output: String,

    /// Name of the output library
    #[arg(long, short, default_value_t = String::from("lucide-icons"))]
    pub name: String,

    /// Tag of the lucide icons release
    pub tag: String
}