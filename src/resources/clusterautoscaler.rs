// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::Resource;

#[derive(Debug, Clone)]
pub struct ClusterAutoscaler {
    manifest: Manifest,
}

impl Resource for ClusterAutoscaler {
    fn from(manifest: Manifest) -> ClusterAutoscaler {
        ClusterAutoscaler { manifest }
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
