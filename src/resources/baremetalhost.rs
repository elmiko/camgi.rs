// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::{GroupKindResource, Resource, ResourceScope};

#[derive(Debug, Clone)]
pub struct BareMetalHost {
    manifest: Manifest,
    ready: bool,
}

#[allow(non_upper_case_globals)]
impl GroupKindResource for BareMetalHost {
    const group: &'static str = "metal3.io";
    const kind: &'static str = "baremetalhost";
    const scope: ResourceScope = ResourceScope::Namespaced;
}

impl Resource for BareMetalHost {
    fn from(manifest: Manifest) -> BareMetalHost {
        BareMetalHost {
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
