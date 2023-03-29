// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::{GroupKindResource, Resource, ResourceScope};

#[derive(Debug, Clone)]
pub struct MachineAutoscaler {
    manifest: Manifest,
}

#[allow(non_upper_case_globals)]
impl GroupKindResource for MachineAutoscaler {
    const group: &'static str = "autoscaling.openshift.io";
    const kind: &'static str = "machineautoscaler";
    const scope: ResourceScope = ResourceScope::Namespaced;
}

impl Resource for MachineAutoscaler {
    fn from(manifest: Manifest) -> MachineAutoscaler {
        MachineAutoscaler { manifest }
    }

    fn name(&self) -> &String {
        &self.manifest.name
    }

    fn raw(&self) -> &String {
        &self.manifest.as_raw()
    }
}
