// Copyright (C) 2026 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::Resource;

#[derive(Debug, Clone)]
pub struct MachineConfiguration {
    manifest: Manifest,
    boot_image_update_degraded: bool,
    boot_image_update_progressing: bool,
}

impl Resource for MachineConfiguration {
    fn from(manifest: Manifest) -> MachineConfiguration {
        let boot_image_update_degraded =
            manifest.has_condition_status("BootImageUpdateDegraded", "True");
        let boot_image_update_progressing =
            manifest.has_condition_status("BootImageUpdateProgressing", "True");
        MachineConfiguration {
            manifest,
            boot_image_update_degraded,
            boot_image_update_progressing,
        }
    }

    fn is_error(&self) -> bool {
        self.boot_image_update_degraded
    }

    fn is_warning(&self) -> bool {
        self.boot_image_update_progressing
    }

    fn name(&self) -> &String {
        &self.manifest.name
    }

    fn raw(&self) -> &String {
        self.manifest.as_raw()
    }

    fn conditions(&self) -> Vec<String> {
        let mut conditions = Vec::new();
        if self.boot_image_update_degraded {
            conditions.push(String::from("BootImageUpdateDegraded"));
        }
        if self.boot_image_update_progressing {
            conditions.push(String::from("BootImageUpdateProgressing"));
        }
        conditions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_machineconfiguration_not_degraded() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/operator.openshift.io/machineconfigurations/cluster.yaml",
        ))
        .unwrap();
        let mc = <MachineConfiguration as Resource>::from(manifest);
        assert_eq!(mc.is_error(), false)
    }

    #[test]
    fn test_machineconfiguration_not_progressing() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/operator.openshift.io/machineconfigurations/cluster.yaml",
        ))
        .unwrap();
        let mc = <MachineConfiguration as Resource>::from(manifest);
        assert_eq!(mc.is_warning(), false)
    }
}
