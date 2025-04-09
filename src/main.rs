use std::{collections::BTreeMap, io::Read, path::Path};

use anyhow::{Context, anyhow};
use clap::Parser;
use cli::Cli;
use info::IconInfo;
use log::{ExtPrintAndExit, Logger};
use reqwest::Client;
use tempfile::{TempDir, tempdir};
use zip::ZipArchive;

mod cli;
mod generate;
mod info;
mod log;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    // let mut pb = spinoff::Spinner::new(spinners::Line, "", None);
    let mut logger = Logger::new();

    logger.next(format!("Getting lucide release for tag {}", cli.tag).as_str());
    let asset_url = get_lucide_release_asset_url(&cli.tag)
        .await
        .unwrap_or_exit(&mut logger);
    logger.next(format!("Downloading asset from {asset_url}").as_str());
    let asset_dir = download_font_asset(&asset_url)
        .await
        .unwrap_or_exit(&mut logger);
    logger.next("Extracting and parsing files from archive");
    let (icons, font_bytes) = extract_archive_files(asset_dir).unwrap_or_exit(&mut logger);

    logger.next("Generating icons enum code");
    let icons_rs = generate::generate_icons_enum(&icons).unwrap_or_exit(&mut logger);
    logger.next("Generating iced icons code");
    let iced_rs = generate::generate_iced_icons(&icons).unwrap_or_exit(&mut logger);
    logger.next("Generating library code");
    let lib_rs = generate::generate_library().unwrap_or_exit(&mut logger);

    let out_dir = Path::new(&cli.output);
    let out_src_dir = out_dir.join("src");
    if !out_dir.exists() || !out_src_dir.exists() {
        std::fs::create_dir_all(&out_src_dir)
            .context("Unable to create output directory")
            .unwrap_or_exit(&mut logger);
    } else if !out_dir.is_dir() || !out_src_dir.is_dir() {
        Err(anyhow!("Output directory is not a directory")).unwrap_or_exit(&mut logger)
    }

    logger.next("Generating Cargo.toml");
    let cargo_toml = generate::generate_cargo_toml(&cli.name, &cli.tag);

    logger.next("Writing output files");
    std::fs::write(out_dir.join("Cargo.toml"), &cargo_toml)
        .context("Unable to write Cargo.toml")
        .unwrap_or_exit(&mut logger);
    std::fs::write(out_src_dir.join("icon.rs"), &icons_rs)
        .context("Unable to write icon.rs")
        .unwrap_or_exit(&mut logger);
    std::fs::write(out_src_dir.join("iced.rs"), &iced_rs)
        .context("Unable to write iced.rs")
        .unwrap_or_exit(&mut logger);
    std::fs::write(out_src_dir.join("lib.rs"), &lib_rs)
        .context("Unable to write lib.rs")
        .unwrap_or_exit(&mut logger);
    std::fs::write(out_dir.join("lucide.ttf"), font_bytes)
        .context("Unable to write lucide.ttf")
        .unwrap_or_exit(&mut logger);

    let current_dir = std::env::current_dir()
        .unwrap_or_default()
        .join(&cli.output);

    logger.finish(format!("Wrote library {} with version {} to {}", cli.name, cli.tag, current_dir.to_string_lossy()));

}

async fn get_lucide_release_asset_url(tag: &str) -> anyhow::Result<String> {
    let octocrab = octocrab::instance();
    let release = octocrab
        .repos("lucide-icons", "lucide")
        .releases()
        .get_by_tag(tag)
        .await
        .context("Unable to get release by tag")?;

    let asset = release
        .assets
        .into_iter()
        .find(|asset| {
            asset.name.starts_with("lucide-font") && asset.content_type == "application/zip"
        })
        .context("No lucide-font release asset found")?;

    Ok(asset.browser_download_url.to_string())
}

async fn download_font_asset(url: &str) -> anyhow::Result<tempfile::TempDir> {
    let response = Client::new()
        .get(url)
        .send()
        .await
        .context("Unable to download font asset")?;
    let bytes = response
        .bytes()
        .await
        .context("Unable to get font asset bytes")?;

    let tmpdir = tempdir().context("Unable to create font asset temporary directory")?;
    std::fs::write(tmpdir.path().join("font.zip"), bytes)
        .context("Unable to write font asset to temporary directory")?;

    Ok(tmpdir)
}

fn extract_archive_files(dir: TempDir) -> anyhow::Result<(BTreeMap<String, IconInfo>, Vec<u8>)> {
    let zip_file =
        std::fs::File::open(dir.path().join("font.zip")).context("Unable to open font zip file")?;
    let mut archive = ZipArchive::new(&zip_file).context("Unable to parse zip archive")?;

    let font_file = archive
        .by_name("lucide-font/lucide.ttf")
        .context("Unable to find font file in archive")?;
    let font_bytes = font_file
        .bytes()
        .collect::<std::io::Result<Vec<_>>>()
        .context("Unable to read font bytes")?;

    let mut info_file = archive
        .by_name("lucide-font/info.json")
        .context("Unable to find font info file in archive")?;
    let mut icons_str = String::new();
    info_file
        .read_to_string(&mut icons_str)
        .context("Unable to read font info file")?;
    let icons: BTreeMap<String, IconInfo> =
        serde_json::from_str(&icons_str).context("Unable to deserialize font info file")?;

    Ok((icons, font_bytes))
}
