// Copyright (C) 2026 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::Resource;

#[derive(Debug, Clone)]
pub struct MachineConfig {
    manifest: Manifest,
}

impl Resource for MachineConfig {
    fn from(manifest: Manifest) -> MachineConfig {
        MachineConfig { manifest }
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
    fn test_machineconfig_name() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/machineconfiguration.openshift.io/machineconfigs/00-master.yaml",
        ))
        .unwrap();
        let mc = <MachineConfig as Resource>::from(manifest);
        assert_eq!(mc.name(), "00-master")
    }
}
