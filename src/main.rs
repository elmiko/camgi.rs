mod html;
mod mustgather;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    // The path to the must-gather.
    path: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mg = mustgather::build_mustgather(cli.path)?;

    let index = html::build_html(mg)?;
    println!("{}", index);
    Ok(())
}
