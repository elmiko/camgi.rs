mod html;
mod manifest;
mod mustgather;
mod resources;

mod prelude {
    pub use crate::html::*;
    pub use crate::manifest::*;
    pub use crate::mustgather::*;
    pub use crate::resources::*;
    pub use anyhow::{anyhow, Result};
}
use crate::prelude::*;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    // The path to the must-gather.
    path: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mg = MustGather::from(cli.path)?;

    let index = Html::from(mg)?;
    println!("{}", index.render());
    Ok(())
}
