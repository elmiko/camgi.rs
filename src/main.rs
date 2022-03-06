// Copyright (C) 2022 Red Hat
// SPDX-License-Identifier: GPL-3.0-or-later

mod html;
mod manifest;
mod mustgather;
mod resources;

mod prelude {
    pub use crate::html::*;
    pub use crate::manifest::*;
    pub use crate::mustgather::*;
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

    if cli.path == "demo" {
        // Special case to render a demo report
        let mg = MustGather {
            path: std::path::PathBuf::from("demo"),
        };
        let index = Html::from(mg)?;

        std::fs::create_dir_all("target/html")?;
        std::fs::write("target/html/index.html", index.render())?;
    } else {
        let mg = MustGather::from(cli.path)?;

        let index = Html::from(mg)?;
        println!("{}", index.render());
    }
    Ok(())
}
