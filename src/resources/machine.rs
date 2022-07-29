// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::Resource;

#[derive(Debug, Clone)]
pub struct Machine {
    manifest: Manifest,
    running: bool,
}

impl Resource for Machine {
    fn from(manifest: Manifest) -> Machine {
        let running = is_running_phase(&manifest);
        Machine { manifest, running }
    }

    fn is_error(&self) -> bool {
        !self.running
    }

    fn name(&self) -> &String {
        &self.manifest.name
    }

    fn raw(&self) -> &String {
        &self.manifest.as_raw()
    }
}

fn is_running_phase(manifest: &Manifest) -> bool {
    let phase = manifest.as_yaml()["status"]["phase"]
        .as_str()
        .unwrap_or("Unknown");
    if phase != "Running" {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::path::PathBuf;

    #[test]
    fn test_machine_is_running_phase_true() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/namespaces/openshift-machine-api/machine.openshift.io/machines/testdata-control-plane-0.yaml",
        ))
        .unwrap();
        assert_eq!(is_running_phase(&manifest), true)
    }

    #[test]
    fn test_machine_is_running_phase_false() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/namespaces/openshift-machine-api/machine.openshift.io/machines/testdata-control-plane-1.yaml"
        )).unwrap();
        assert_eq!(is_running_phase(&manifest), false)
    }
}
