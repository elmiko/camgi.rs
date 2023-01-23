// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::{GroupKindResource, Resource, ResourceScope};

#[derive(Debug, Clone)]
pub struct Node {
    manifest: Manifest,
    ready: bool,
}

#[allow(non_upper_case_globals)]
impl GroupKindResource for Node {
    const group: &'static str = "core";
    const kind: &'static str = "node";
    const scope: ResourceScope = ResourceScope::Cluster;
}

impl Resource for Node {
    fn from(manifest: Manifest) -> Node {
        let ready = manifest.has_condition_status("Ready", "True");
        Node { manifest, ready }
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
