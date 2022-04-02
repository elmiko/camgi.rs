// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::Resource;

pub struct MachineSet {
    manifest: Manifest,
    autoscaling: bool,
    replicas: i64,
}

impl MachineSet {
    pub fn is_autoscaling(&self) -> bool {
        self.autoscaling
    }

    pub fn replicas(&self) -> i64 {
        self.replicas
    }
}

impl Resource for MachineSet {
    fn from(manifest: Manifest) -> MachineSet {
        let autoscaling = has_autoscaling_annotations(&manifest);
        let replicas = status_replicas(&manifest);
        MachineSet {
            manifest,
            autoscaling,
            replicas,
        }
    }

    fn is_error(&self) -> bool {
        false
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

fn has_autoscaling_annotations(manifest: &Manifest) -> bool {
    if manifest.as_yaml()["metadata"]["annotations"]
        ["machine.openshift.io/cluster-api-autoscaler-node-group-min-size"]
        .is_badvalue()
        && manifest.as_yaml()["metadata"]["annotations"]
            ["machine.openshift.io/cluster-api-autoscaler-node-group-max-size"]
            .is_badvalue()
    {
        false
    } else {
        true
    }
}

fn status_replicas(manifest: &Manifest) -> i64 {
    if manifest.as_yaml()["status"]["replicas"].is_badvalue() {
        -1
    } else {
        match manifest.as_yaml()["status"]["replicas"].as_i64() {
            Some(v) => v,
            None => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::path::PathBuf;

    #[test]
    fn test_machineset_has_autoscaling_annotations_true() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/namespaces/openshift-machine-api/machine.openshift.io/machinesets/testdata-compute-region-2.yaml",
        ))
        .unwrap();
        assert_eq!(has_autoscaling_annotations(&manifest), true)
    }

    #[test]
    fn test_machineset_has_autoscaling_annotations_false() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/namespaces/openshift-machine-api/machine.openshift.io/machinesets/testdata-compute-region-1.yaml",
        )).unwrap();
        assert_eq!(has_autoscaling_annotations(&manifest), false)
    }

    #[test]
    fn test_machineset_status_replicas() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/namespaces/openshift-machine-api/machine.openshift.io/machinesets/testdata-compute-region-2.yaml",
        )).unwrap();
        assert_eq!(status_replicas(&manifest), 0)
    }
}
