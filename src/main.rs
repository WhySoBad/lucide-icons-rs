use std::{collections::BTreeMap, io::Read, path::Path, process::exit};

use anyhow::{anyhow, Context};
use clap::Parser;
use cli::Cli;
use reqwest::Client;
use serde::Deserialize;
use spinoff::spinners;
use tempfile::{tempdir, TempDir};
use zip::ZipArchive;

mod cli;
mod codegen;
mod github;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut pb = spinoff::Spinner::new(spinners::Dots, "", None);

    pb.update_text("Getting lucide github release");
    let asset_url = get_lucide_release_asset_url(&cli.tag).await.spinner(&mut pb);
    pb.update_text("Downloading asset from github release");
    let asset_dir = download_font_asset(&asset_url).await.spinner(&mut pb);
    pb.update_text("Extracting and parsing files from archive");
    let (icons, font_bytes) = extract_archive_files(asset_dir).spinner(&mut pb);

    pb.update_text("Generating icons enum code");
    let icons_enum = codegen::generate_icons_enum(&icons).spinner(&mut pb);
    pb.update_text("Generating iced icons code");
    let iced_icons = codegen::generate_iced_icons(&icons).spinner(&mut pb);

    let out_dir = Path::new(&cli.output);
    if !out_dir.exists() {
        std::fs::create_dir_all(out_dir).context("Unable to create output directory").spinner(&mut pb);
    } else if !out_dir.is_dir() {
        Err(anyhow!("Output directory is not a directory")).spinner(&mut pb)
    }

    std::fs::write(out_dir.join("icons.rs"), &icons_enum).context("Unable to write icons.rs").spinner(&mut pb);
    std::fs::write(out_dir.join("iced.rs"), &iced_icons).context("Unable to write iced.rs").spinner(&mut pb);
    std::fs::write(out_dir.join("lucide.ttf"), font_bytes).context("Unable to write lucide.ttf").spinner(&mut pb);

    let current_dir = std::env::current_dir().unwrap_or_default().join(&cli.output);
    pb.success(&format!("Successfully wrote library to {}", current_dir.to_string_lossy()));
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct IconInfo {
    encoded_code: String,
    prefix: String,
    class_name: String,
    unicode: String,
}

impl IconInfo {
    pub fn unicode(&self) -> char {
        let bytes = u16::from_str_radix(&self.encoded_code.as_str()[1..], 16)
            .expect("should parse icon unicode as u16");
        char::from_u32(bytes as u32).expect("should be a vaild unicode character")
    }
}

async fn get_lucide_release_asset_url(tag: &str) -> anyhow::Result<String> {
    let octocrab = octocrab::instance();
    let release = octocrab
        .repos("lucide-icons", "lucide")
        .releases()
        .get_by_tag(tag)
        .await
        .context("Unable to get release by tag")?;

    let asset = release.assets.into_iter().find(|asset| {
        asset.name.starts_with("lucide-font") && asset.content_type == "application/zip"
    }).context("No lucide-font release asset found")?;

    Ok(asset.browser_download_url.to_string())
}

async fn download_font_asset(url: &str) -> anyhow::Result<tempfile::TempDir> {
    let response = Client::new().get(url).send().await.context("Unable to download font asset")?;
    let bytes = response.bytes().await.context("Unable to get font asset bytes")?;

    let tmpdir = tempdir().context("Unable to create font asset temporary directory")?;
    std::fs::write(tmpdir.path().join("font.zip"), bytes).context("Unable to write font asset to temporary directory")?;

    Ok(tmpdir)
}

fn extract_archive_files(dir: TempDir) -> anyhow::Result<(BTreeMap<String, IconInfo>, Vec<u8>)> {
    let zip_file = std::fs::File::open(dir.path().join("font.zip")).context("Unable to open font zip file")?;
    let mut archive = ZipArchive::new(&zip_file).context("Unable to parse zip archive")?;

    let font_file = archive.by_name("lucide-font/lucide.ttf").context("Unable to find font file in archive")?;
    let font_bytes = font_file.bytes().collect::<std::io::Result<Vec<_>>>().context("Unable to read font bytes")?;

    let mut info_file = archive.by_name("lucide-font/info.json").context("Unable to find font info file in archive")?;
    let mut icons_str = String::new();
    info_file.read_to_string(&mut icons_str).context("Unable to read font info file")?;
    let icons: BTreeMap<String, IconInfo> =
        serde_json::from_str(&icons_str).context("Unable to deserialize font info file")?;

    Ok((icons, font_bytes))
}

trait ExtPrintAndExit<T> {
    fn spinner(self, spinner: &mut spinoff::Spinner) -> T;
}

impl<T> ExtPrintAndExit<T> for anyhow::Result<T> {
    fn spinner(self, spinner: &mut spinoff::Spinner) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                spinner.fail(&format!("{err}"));
                exit(1)
            },
        }
    }
}