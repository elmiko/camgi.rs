// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::Resource;

#[derive(Debug, Clone)]
pub struct ControlPlaneMachineSet {
    manifest: Manifest,
    ready: bool,
}

impl Resource for ControlPlaneMachineSet {
    fn from(manifest: Manifest) -> ControlPlaneMachineSet {
        ControlPlaneMachineSet {
            manifest,
            ready: true,
        }
    }

    fn is_error(&self) -> bool {
        !self.ready
    }

    fn name(&self) -> &String {
        &self.manifest.name
    }

    fn raw(&self) -> &String {
        &self.manifest.as_raw()
    }
}
