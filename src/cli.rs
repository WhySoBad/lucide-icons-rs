use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// Path to the `info.json` file
    #[arg(long, short)]
    pub info_path: PathBuf,

    /// Path to the `lucide.ttf` font file
    #[arg(long, short)]
    pub font_path: PathBuf,

    /// Directory to write the output library to
    #[arg(long, short)]
    pub output_directory: PathBuf,

    /// Lucide icons version
    pub version: String
}