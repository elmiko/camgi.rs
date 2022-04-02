// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::Resource;
use yaml_rust::Yaml;

#[derive(Debug, Clone)]
pub struct Node {
    manifest: Manifest,
    ready: bool,
}

impl Resource for Node {
    fn from(manifest: Manifest) -> Node {
        let ready = is_ready_condition(&manifest);
        Node { manifest, ready }
    }

    fn is_error(&self) -> bool {
        !self.ready
    }

    fn name(&self) -> &String {
        &self.manifest.name
    }

    fn raw(&self) -> &String {
        &self.manifest.as_raw()
    }

    fn safename(&self) -> &String {
        &self.manifest.safename
    }
}

fn is_ready_condition(manifest: &Manifest) -> bool {
    let empty = Vec::<Yaml>::new();
    let conditions = manifest.as_yaml()["status"]["conditions"]
        .as_vec()
        .unwrap_or(&empty);
    for c in conditions {
        if c["type"].as_str().unwrap() == "Ready" && c["status"].as_str().unwrap() == "False" {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::path::PathBuf;

    #[test]
    fn test_node_is_ready_condition_true() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/core/nodes/ip-10-0-0-1.control.plane.yaml"
        )).unwrap();
        assert_eq!(is_ready_condition(&manifest), true)
    }

    #[test]
    fn test_node_is_ready_condition_false() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/core/nodes/ip-10-0-0-2.control.plane.yaml"
        )).unwrap();
        assert_eq!(is_ready_condition(&manifest), false)
    }
}
