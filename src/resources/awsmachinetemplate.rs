// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::Resource;

#[derive(Debug, Clone)]
pub struct AWSMachineTemplate {
    manifest: Manifest,
}

impl Resource for AWSMachineTemplate {
    fn from(manifest: Manifest) -> AWSMachineTemplate {
        AWSMachineTemplate { manifest }
    }

    fn name(&self) -> &String {
        &self.manifest.name
    }

    fn raw(&self) -> &String {
        self.manifest.as_raw()
    }
}
