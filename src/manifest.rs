use anyhow::{anyhow, Result};
use std::fs;
use std::path::PathBuf;
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug)]
pub struct Manifest {
    yaml: Yaml,
}

impl Manifest {
    pub fn from(path: PathBuf) -> Result<Manifest> {
        if !path.is_file() {
            return Err(anyhow!("Path is not a file {}", path.as_path().display()));
        }
        if path.is_dir() {
            return Err(anyhow!("Path is a directory {}", path.as_path().display()));
        }

        let raw = fs::read_to_string(path.as_path())?;
        let mut docs = YamlLoader::load_from_str(&raw)?;
        match docs.is_empty() {
            true => Err(anyhow!(
                "No YAML documents found in path {}",
                path.as_path().display()
            )),
            false => Ok(Manifest {
                yaml: docs.remove(0),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_from_unknown_error() {
        let observed = Manifest::from(PathBuf::from(""));
        assert!(observed.is_err())
    }

    #[test]
    fn test_manifest_from_not_a_file() {
        let observed = Manifest::from(PathBuf::from(
            "testdata/must-gather-invalid/does-not-exist.yaml",
        ));
        assert!(observed.is_err())
    }

    #[test]
    fn test_manifest_from_is_a_directory() {
        let observed = Manifest::from(PathBuf::from("testdata/must-gather-valid"));
        assert!(observed.is_err())
    }

    #[test]
    fn test_manifest_from_empty_file() {
        let observed = Manifest::from(PathBuf::from("testdata/must-gather-invalid/empty.yaml"));
        assert!(observed.is_err())
    }
}
