// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::Resource;

#[derive(Debug, Clone)]
pub struct CAPICluster {
    manifest: Manifest,
    ready: bool,
}

impl Resource for CAPICluster {
    fn from(manifest: Manifest) -> CAPICluster {
        let ready = manifest.has_condition_status("InfrastructureReady", "True");
        CAPICluster { manifest, ready }
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
    fn test_capicluster_ready() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/namespaces/openshift-cluster-api/cluster.x-k8s.io/clusters/testdata.yaml",
        )).unwrap();
        let cluster = <CAPICluster as Resource>::from(manifest);
        assert_eq!(cluster.is_error(), false)
    }

    #[test]
    fn test_capicluster_not_ready() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/namespaces/openshift-cluster-api/cluster.x-k8s.io/clusters/testdata-not-ready.yaml",
        )).unwrap();
        let cluster = <CAPICluster as Resource>::from(manifest);
        assert_eq!(cluster.is_error(), true)
    }
}
