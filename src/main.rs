use clap::Parser;

mod embed;
mod process;

#[derive(Debug, Parser)]
struct CliArgs {
    /// Path to the markdown file containing your page
    #[clap(short, long)]
    file: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let cli_args = CliArgs::parse();

    Ok(())
}
