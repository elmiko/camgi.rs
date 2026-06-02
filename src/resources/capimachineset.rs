// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::Resource;

#[derive(Debug, Clone)]
pub struct CAPIMachineSet {
    manifest: Manifest,
    ready: bool,
}

impl Resource for CAPIMachineSet {
    fn from(manifest: Manifest) -> CAPIMachineSet {
        let ready = manifest.has_condition_status("MachinesReady", "True");
        CAPIMachineSet { manifest, ready }
    }

    fn is_error(&self) -> bool {
        !self.ready
    }

    fn name(&self) -> &String {
        &self.manifest.name
    }

    fn raw(&self) -> &String {
        self.manifest.as_raw()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_capimachineset_ready() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/namespaces/openshift-cluster-api/cluster.x-k8s.io/machinesets/testdata-worker-region-1.yaml",
        )).unwrap();
        let ms = <CAPIMachineSet as Resource>::from(manifest);
        assert_eq!(ms.is_error(), false)
    }

    #[test]
    fn test_capimachineset_not_ready() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/namespaces/openshift-cluster-api/cluster.x-k8s.io/machinesets/testdata-worker-empty.yaml",
        )).unwrap();
        let ms = <CAPIMachineSet as Resource>::from(manifest);
        assert_eq!(ms.is_error(), true)
    }
}
