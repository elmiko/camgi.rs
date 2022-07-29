// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::Resource;

#[derive(Debug, Clone)]
pub struct MachineAutoscaler {
    manifest: Manifest,
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
