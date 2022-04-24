// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use std::fs;
use std::path::PathBuf;
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug, Clone)]
pub struct Manifest {
    pub name: String,
    pub safename: String,
    raw: String,
    yaml: Yaml,
}

impl Manifest {
    pub fn new() -> Manifest {
        Manifest {
            name: String::new(),
            safename: String::new(),
            raw: String::new(),
            yaml: Yaml::Null,
        }
    }

    pub fn from(path: PathBuf) -> Result<Manifest> {
        if !path.is_file() {
            return Err(anyhow!("Path is not a file {}", path.as_path().display()));
        }
        if path.is_dir() {
            return Err(anyhow!("Path is a directory {}", path.as_path().display()));
        }

        let raw = fs::read_to_string(path.as_path())?;
        let mut docs = YamlLoader::load_from_str(&raw)?;

        if docs.is_empty() {
            Err(anyhow!(
                "No YAML documents found in path {}",
                path.as_path().display()
            ))
        } else {
            let yaml = docs.remove(0);
            let name = match yaml["metadata"]["name"].as_str() {
                Some(n) => String::from(n),
                None => String::from("Unknown"),
            };
            let safename = render_safename(name.as_str());
            Ok(Manifest {
                name,
                raw,
                safename,
                yaml,
            })
        }
    }

    pub fn as_yaml(&self) -> &Yaml {
        &self.yaml
    }

    pub fn as_raw(&self) -> &String {
        &self.raw
    }

    /// Return true if the manifest has the condition type.
    pub fn has_condition(&self, condition: &str) -> bool {
        let empty = Vec::<Yaml>::new();
        let conditions = self.as_yaml()["status"]["conditions"]
            .as_vec()
            .unwrap_or(&empty);
        let matchedconditions: Vec<&Yaml> = conditions
            .iter()
            .filter(|c| c["type"].as_str().unwrap_or("") == condition)
            .collect();
        !matchedconditions.is_empty()
    }

    /// Return true if the manfiest has the condition type with the specified status.
    ///
    /// If the manifest has a `status.conditions` list, this function will iterate
    /// through them attempting to match the condition type and status strings.
    pub fn has_condition_status(&self, condition: &str, status: &str) -> bool {
        let empty = Vec::<Yaml>::new();
        let conditions = self.as_yaml()["status"]["conditions"]
            .as_vec()
            .unwrap_or(&empty);
        for c in conditions {
            if c["type"].as_str().unwrap_or("") == condition
                && c["status"].as_str().unwrap_or("") == status
            {
                return true;
            }
        }

        false
    }
}

pub fn render_safename(original: &str) -> String {
    original.replace('.', "-").replace(':', "_")
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

    #[test]
    fn test_manifest_as_yaml() {
        let expected = "Node";
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/core/nodes/ip-10-0-0-1.control.plane.yaml"
        )).unwrap();
        let observed = &manifest.as_yaml()["kind"];
        assert_eq!(observed.as_str().unwrap(), expected)
    }

    #[test]
    fn test_manifest_as_raw() {
        let expected = include_str!(
            "../testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/core/nodes/ip-10-0-0-1.control.plane.yaml");
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/core/nodes/ip-10-0-0-1.control.plane.yaml"
        )).unwrap();
        let observed = manifest.as_raw();
        assert_eq!(observed, expected)
    }

    #[test]
    fn test_manifest_name() {
        let expected = String::from("ip-10-0-0-1.control.plane");
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/core/nodes/ip-10-0-0-1.control.plane.yaml"
        )).unwrap();
        assert_eq!(manifest.name, expected)
    }

    #[test]
    fn test_manifest_has_condition_status_true() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/core/nodes/ip-10-0-0-1.control.plane.yaml"
        )).unwrap();
        assert_eq!(manifest.has_condition_status("Ready", "True"), true)
    }

    #[test]
    fn test_manifest_has_condition_status_false() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/core/nodes/ip-10-0-0-1.control.plane.yaml"
        )).unwrap();
        assert_eq!(manifest.has_condition_status("PIDPressure", "True"), false)
    }

    #[test]
    fn test_manifest_has_condition_status_false_nonexistant() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/core/nodes/ip-10-0-0-1.control.plane.yaml"
        )).unwrap();
        assert_eq!(manifest.has_condition_status("foo", "bar"), false)
    }

    #[test]
    fn test_manifest_has_condition_true() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/core/nodes/ip-10-0-0-1.control.plane.yaml"
        )).unwrap();
        assert_eq!(manifest.has_condition("Ready"), true)
    }

    #[test]
    fn test_manifest_has_condition_false() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/core/nodes/ip-10-0-0-1.control.plane.yaml"
        )).unwrap();
        assert_eq!(manifest.has_condition("FooBar"), false)
    }

    #[test]
    fn test_render_safename_hyphen() {
        let expected = String::from("ip-10-0-0-1-control-plane");
        let observed = render_safename("ip-10-0-0-1.control.plane");
        assert_eq!(observed, expected)
    }

    #[test]
    fn test_render_safename_colon() {
        let expected = String::from("ip-10-0-0-1_control_plane");
        let observed = render_safename("ip-10-0-0-1:control:plane");
        assert_eq!(observed, expected)
    }
}
