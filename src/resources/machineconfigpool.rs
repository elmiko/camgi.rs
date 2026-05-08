// Copyright (C) 2026 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::Resource;

#[derive(Debug, Clone)]
pub struct MachineConfigPool {
    manifest: Manifest,
    degraded: bool,
    updating: bool,
}

impl Resource for MachineConfigPool {
    fn from(manifest: Manifest) -> MachineConfigPool {
        let degraded = manifest.has_condition_status("Degraded", "True");
        let updating = manifest.has_condition_status("Updating", "True");
        MachineConfigPool {
            manifest,
            degraded,
            updating,
        }
    }

    fn is_error(&self) -> bool {
        self.degraded
    }

    fn is_warning(&self) -> bool {
        self.updating
    }

    fn name(&self) -> &String {
        &self.manifest.name
    }

    fn raw(&self) -> &String {
        self.manifest.as_raw()
    }

    fn conditions(&self) -> Vec<String> {
        let mut conditions = Vec::new();
        if self.degraded {
            conditions.push(String::from("Degraded"));
        }
        if self.updating {
            conditions.push(String::from("Updating"));
        }
        conditions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_machineconfigpool_degraded_false() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/machineconfiguration.openshift.io/machineconfigpools/master.yaml",
        ))
        .unwrap();
        let mcp = <MachineConfigPool as Resource>::from(manifest);
        assert_eq!(mcp.is_error(), false)
    }

    #[test]
    fn test_machineconfigpool_degraded_true() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/machineconfiguration.openshift.io/machineconfigpools/worker.yaml",
        ))
        .unwrap();
        let mcp = <MachineConfigPool as Resource>::from(manifest);
        assert_eq!(mcp.is_error(), true)
    }

    #[test]
    fn test_machineconfigpool_updating_true() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/machineconfiguration.openshift.io/machineconfigpools/worker.yaml",
        ))
        .unwrap();
        let mcp = <MachineConfigPool as Resource>::from(manifest);
        assert_eq!(mcp.is_warning(), false)
    }
}
