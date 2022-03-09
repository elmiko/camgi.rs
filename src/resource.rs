use anyhow::{anyhow, Result};
use std::fs;
use std::path::PathBuf;
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug)]
pub struct Resource {
    raw: String,
    pub yaml: Yaml,
}

impl Resource {
    pub fn from(path: PathBuf) -> Result<Resource> {
        if !path.is_file() {
            return Err(anyhow!("Path is not a file {}", path.as_path().display()));
        }
        if path.is_dir() {
            return Err(anyhow!("Path is a directory {}", path.as_path().display()));
        }

        let raw = fs::read_to_string(path.as_path())?;
        let mut docs = YamlLoader::load_from_str(&raw)?;
        let res = Resource {
            raw: raw,
            yaml: docs.remove(0),
        };
        Ok(res)
    }

    pub fn name(&self) -> String {
        match self.yaml["metadata"]["name"].as_str() {
            Some(n) => String::from(n),
            None => String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_new_unknown_error() {
        let observed = Resource::from(PathBuf::from(""));
        assert!(observed.is_err())
    }

    #[test]
    fn test_resource_new_not_a_file() {
        let observed = Resource::from(PathBuf::from(
            "testdata/must-gather-invalid/does-not-exist.yaml",
        ));
        assert!(observed.is_err())
    }

    #[test]
    fn test_resource_new_is_a_directory() {
        let observed = Resource::from(PathBuf::from("testdata/must-gather-valid"));
        assert!(observed.is_err())
    }

    #[test]
    fn test_resource_new_raw() {
        let expected = include_str!(
            "../testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/core/nodes/ip-10-0-0-1.control.plane.yaml"
        );
        let observed = Resource::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/core/nodes/ip-10-0-0-1.control.plane.yaml"
        )).unwrap().raw;
        assert_eq!(observed, expected)
    }

    #[test]
    fn test_resource_name() {
        let observed = Resource::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/core/nodes/ip-10-0-0-1.control.plane.yaml"
        )).unwrap().name();
        assert_eq!(observed, String::from("ip-10-0-0-1.control.plane"))
    }

    #[test]
    fn test_resource_name_empty() {
        let observed = Resource::from(PathBuf::from(
            "testdata/must-gather-invalid/ip-10-0-0-1.control.plane.yaml",
        ))
        .unwrap()
        .name();
        assert_eq!(observed, String::new())
    }
}
