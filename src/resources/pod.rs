// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::{GroupKindResource, Resource, ResourceScope};

#[derive(Debug, Clone)]
pub struct Pod {
    manifest: Manifest,
    pub containers: Vec<Container>,
}

#[allow(non_upper_case_globals)]
impl GroupKindResource for Pod {
    const group: &'static str = "";
    const kind: &'static str = "pod";
    const scope: ResourceScope = ResourceScope::Namespaced;
}

impl Pod {
    pub fn new() -> Pod {
        Pod {
            manifest: Manifest::new(),
            containers: Vec::new(),
        }
    }

    pub fn push_container(&mut self, container: Container) {
        self.containers.push(container);
    }
}

impl Resource for Pod {
    fn from(manifest: Manifest) -> Pod {
        let containers = Vec::new();
        Pod {
            manifest,
            containers,
        }
    }

    fn name(&self) -> &String {
        &self.manifest.name
    }

    fn raw(&self) -> &String {
        &self.manifest.as_raw()
    }
}

/// Holds the name and raw log files of a container within a pod.
#[derive(Debug, Clone)]
pub struct Container {
    pub name: String,
    pub current_log: String,
}
