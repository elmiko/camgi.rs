// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::Resource;

#[derive(Debug, Clone)]
pub struct AWSMachine {
    manifest: Manifest,
    ready: bool,
}

impl Resource for AWSMachine {
    fn from(manifest: Manifest) -> AWSMachine {
        let ready = manifest.as_yaml()["status"]["ready"]
            .as_bool()
            .unwrap_or(false);
        AWSMachine { manifest, ready }
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
    fn test_awsmachine_ready() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/namespaces/openshift-cluster-api/infrastructure.cluster.x-k8s.io/awsmachines/testdata-worker-0.yaml",
        )).unwrap();
        let machine = <AWSMachine as Resource>::from(manifest);
        assert_eq!(machine.is_error(), false)
    }

    #[test]
    fn test_awsmachine_not_ready() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/namespaces/openshift-cluster-api/infrastructure.cluster.x-k8s.io/awsmachines/testdata-worker-not-ready.yaml",
        )).unwrap();
        let machine = <AWSMachine as Resource>::from(manifest);
        assert_eq!(machine.is_error(), true)
    }
}
