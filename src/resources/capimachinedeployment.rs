// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::Resource;

#[derive(Debug, Clone)]
pub struct CAPIMachineDeployment {
    manifest: Manifest,
    ready: bool,
}

impl Resource for CAPIMachineDeployment {
    fn from(manifest: Manifest) -> CAPIMachineDeployment {
        let ready = manifest.has_condition_status("Available", "True");
        CAPIMachineDeployment { manifest, ready }
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
    fn test_capimachinedeployment_available() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/namespaces/openshift-cluster-api/cluster.x-k8s.io/machinedeployments/testdata-available.yaml",
        )).unwrap();
        let md = <CAPIMachineDeployment as Resource>::from(manifest);
        assert_eq!(md.is_error(), false)
    }

    #[test]
    fn test_capimachinedeployment_not_available() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/namespaces/openshift-cluster-api/cluster.x-k8s.io/machinedeployments/testdata-not-available.yaml",
        )).unwrap();
        let md = <CAPIMachineDeployment as Resource>::from(manifest);
        assert_eq!(md.is_error(), true)
    }
}
