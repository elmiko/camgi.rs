use std::fs;
use std::path::PathBuf;

pub struct MustGather {
    pub path: PathBuf,
}

impl MustGather {
    pub fn title(&self) -> String {
        self.path.to_str().unwrap().to_string()
    }
}

pub fn build_mustgather(path: String) -> MustGather {
    let mgpath = match find_must_gather_root(path) {
        Some(path) => path,
        None => panic!("Cannot determine root of must-gather"),
    };

    MustGather { path: mgpath }
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
fn find_must_gather_root(path: String) -> Option<PathBuf> {
    let orig = PathBuf::from(&path);
    let vpath: PathBuf = [String::from(&path), String::from("version")]
        .iter()
        .collect();
    let npath: PathBuf = [String::from(&path), String::from("namespaces")]
        .iter()
        .collect();
    let csrpath: PathBuf = [
        String::from(&path),
        String::from("cluster-scoped-resources"),
    ]
    .iter()
    .collect();

    if vpath.is_file() || (npath.is_dir() && csrpath.is_dir()) {
        let canonical = fs::canonicalize(orig);
        return Some(canonical.unwrap());
    }

    let directories: Vec<PathBuf> = fs::read_dir(orig)
        .unwrap()
        .into_iter()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .filter(|r| r.is_dir())
        .collect();

    if directories.len() == 1 {
        find_must_gather_root(String::from(directories[0].to_str().unwrap()))
    } else {
        None
    }
}