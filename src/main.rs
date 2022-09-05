#![allow(unused)]

use clap::Parser;

mod embed;
mod processor;

#[derive(Debug, Parser)]
struct CliArgs {
    /// Path to the markdown file containing your page
    #[clap(short, long)]
    file: std::path::PathBuf,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let cli_args = CliArgs::parse();

    Ok(())
}

