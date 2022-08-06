// Copyright (C) 2022 Red Hat, Inc.
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
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

use crate::prelude::*;

use clap::Parser;
use flate2::read::GzDecoder;
use tar::Archive;
use tempdir::TempDir;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The path to the must-gather.
    path: String,
    /// Open a must-gather archive in tar format
    #[clap(long)]
    tar: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.path == "demo" {
        // Special case to render a demo report
        let mg =
            MustGather::from("testdata/must-gather-valid/sample-openshift-release/".to_string())?;
        let index = Html::from(mg)?;

        std::fs::create_dir_all("target/html")?;
        std::fs::write("target/html/index.html", index.render())?;
    } else {
        let mg;
        let tmp_dir;
        if cli.tar {
            tmp_dir = match extract_tar(&cli.path) {
                Ok(dir) => dir,
                Err(error) => {
                    eprintln!("Could not extract tar file: {}", cli.path);
                    return Err(error);
                }
            };
            mg = MustGather::from(tmp_dir.path().to_str().unwrap().to_owned())?;
        } else {
            mg = MustGather::from(cli.path)?;
        }

        let index = Html::from(mg)?;
        println!("{}", index.render());
    }
    Ok(())
}

// extract_tar extracts tar or tar.gz file into a temporary directory.
// The temporary directory has lifetime of the returned TempDir object.
fn extract_tar(path: &str) -> Result<TempDir> {
    let mut tar_file = File::open(path)?;

    // Infer file type from magic number
    let mut buf = [0; 512];
    tar_file.read_exact(&mut buf)?;
    tar_file.seek(SeekFrom::Start(0))?;
    let kind = match infer::get(&buf) {
        Some(kind) => kind,
        None => Err(anyhow!("Unknown file type"))?,
    };

    // If the file is gzipped, wrap it in a GzDecoder.
    let reader: Box<dyn Read> = match kind.mime_type() {
        "application/gzip" => Box::new(GzDecoder::new(tar_file)),
        "application/x-tar" => Box::new(tar_file),
        _ => Err(anyhow!("Unsupported file type"))?,
    };

    // Unpack the tar file into a temporary directory.
    let mut archive = Archive::new(reader);
    let tmp_dir = TempDir::new("camgi-must-gather")?;
    archive.unpack(tmp_dir.path())?;
    Ok(tmp_dir)
}
