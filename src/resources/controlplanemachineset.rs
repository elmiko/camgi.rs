// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::{GroupKindResource, Resource, ResourceScope};

#[derive(Debug, Clone)]
pub struct ControlPlaneMachineSet {
    manifest: Manifest,
    ready: bool,
}

#[allow(non_upper_case_globals)]
impl GroupKindResource for ControlPlaneMachineSet {
    const group: &'static str = "machine.openshift.io";
    const kind: &'static str = "controlplanemachineset";
    const scope: ResourceScope = ResourceScope::Namespaced;
}

impl Resource for ControlPlaneMachineSet {
    fn from(manifest: Manifest) -> ControlPlaneMachineSet {
        ControlPlaneMachineSet {
            manifest,
            ready: true,
        }
    }

    fn name(&self) -> &String {
        &self.manifest.name
    }

    fn raw(&self) -> &String {
        &self.manifest.as_raw()
    }

    fn is_error(&self) -> bool {
        !self.ready
    }
}
