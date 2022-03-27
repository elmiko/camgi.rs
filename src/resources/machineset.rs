// Copyright (C) 2022 Red Hat
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::Resource;

pub struct MachineSet {
    manifest: Manifest,
}

impl Resource for MachineSet {
    fn from(manifest: Manifest) -> MachineSet {
        MachineSet { manifest }
    }

    fn is_error(&self) -> bool {
        false
    }

    fn name(&self) -> &String {
        &self.manifest.name
    }

    fn raw(&self) -> &String {
        &self.manifest.as_raw()
    }

    fn safename(&self) -> &String {
        &self.manifest.safename
    }
}
