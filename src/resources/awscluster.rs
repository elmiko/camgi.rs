// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::Resource;

#[derive(Debug, Clone)]
pub struct AWSCluster {
    manifest: Manifest,
    ready: bool,
}

impl Resource for AWSCluster {
    fn from(manifest: Manifest) -> AWSCluster {
        let ready = manifest.as_yaml()["status"]["ready"]
            .as_bool()
            .unwrap_or(false);
        AWSCluster { manifest, ready }
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
    fn test_awscluster_ready() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/namespaces/openshift-cluster-api/infrastructure.cluster.x-k8s.io/awsclusters/testdata.yaml",
        )).unwrap();
        let cluster = <AWSCluster as Resource>::from(manifest);
        assert_eq!(cluster.is_error(), false)
    }

    #[test]
    fn test_awscluster_not_ready() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/namespaces/openshift-cluster-api/infrastructure.cluster.x-k8s.io/awsclusters/testdata-not-ready.yaml",
        )).unwrap();
        let cluster = <AWSCluster as Resource>::from(manifest);
        assert_eq!(cluster.is_error(), true)
    }
}
