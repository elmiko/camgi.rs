use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    // The path to the must-gather.
    path: String,
}

fn main() {
    let cli = Cli::parse();

    let mgpath = find_must_gather_root(cli.path).unwrap();
    println!("must-gather path: {:?}", mgpath);
}

/// Find the root of a must-gather directory structure given a path.
///
/// Finding the root of the must-gather is accomplished through the following steps:
/// 1. look for a `version` file in the current path, if it exists return current path.
/// 2. look for the directories `namespaces` and `cluster-scoped-resources` in the current path,
///    if they exist, return the current path.
/// 3. if there is a single subdirectory in the path, recursively run this function on it and
///    return the result.
/// 4. return an error
fn find_must_gather_root(path: String) -> Result<PathBuf, String> {
    let vpath: PathBuf = [String::from(&path), String::from("version")].iter().collect();
    let npath: PathBuf = [String::from(&path), String::from("namespaces")].iter().collect();
    let csrpath : PathBuf = [String::from(&path), String::from("cluster-scoped-resources")].iter().collect();

    if vpath.is_file() || (npath.is_dir() && csrpath.is_dir()) {
        return Ok(PathBuf::from(path))
    }

    Err(String::from("No must-gather root found"))
}
