// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::Resource;

#[derive(Debug, Clone)]
pub struct CAPIMachine {
    manifest: Manifest,
    phase: String,
    conditions: Vec<String>,
}

impl Resource for CAPIMachine {
    fn from(manifest: Manifest) -> CAPIMachine {
        let phase = manifest.as_yaml()["status"]["phase"]
            .as_str()
            .unwrap_or("Unknown")
            .to_string();
        let conditions = collect_conditions(&manifest);
        CAPIMachine {
            manifest,
            phase,
            conditions,
        }
    }

    fn is_error(&self) -> bool {
        self.phase != "Running"
    }

    fn name(&self) -> &String {
        &self.manifest.name
    }

    fn raw(&self) -> &String {
        self.manifest.as_raw()
    }

    fn conditions(&self) -> Vec<String> {
        self.conditions.clone()
    }
}

fn collect_conditions(manifest: &Manifest) -> Vec<String> {
    let mut conditions = Vec::new();
    if !manifest.has_condition_status("Ready", "True") {
        conditions.push(String::from("NotReady"));
    }
    if manifest.has_condition_status("InfrastructureReady", "False") {
        conditions.push(String::from("InfraNotReady"));
    }
    conditions
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_capimachine_phase_running() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/namespaces/openshift-cluster-api/cluster.x-k8s.io/machines/testdata-worker-0.yaml",
        )).unwrap();
        let machine = <CAPIMachine as Resource>::from(manifest);
        assert_eq!(machine.is_error(), false);
    }

    #[test]
    fn test_capimachine_conditions_ready() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/namespaces/openshift-cluster-api/cluster.x-k8s.io/machines/testdata-worker-0.yaml",
        )).unwrap();
        assert_eq!(collect_conditions(&manifest), Vec::<String>::new())
    }

    #[test]
    fn test_capimachine_phase_failed() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/namespaces/openshift-cluster-api/cluster.x-k8s.io/machines/testdata-worker-failed.yaml",
        )).unwrap();
        let machine = <CAPIMachine as Resource>::from(manifest);
        assert_eq!(machine.is_error(), true);
    }

    #[test]
    fn test_capimachine_conditions_not_ready() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/namespaces/openshift-cluster-api/cluster.x-k8s.io/machines/testdata-worker-failed.yaml",
        )).unwrap();
        assert_eq!(
            collect_conditions(&manifest),
            vec!["NotReady", "InfraNotReady"]
        )
    }
}
