// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::manifest::render_safename;
use crate::prelude::*;
use crate::resources::Resource;

#[derive(Debug, Clone)]
pub struct Pod {
    manifest: Manifest,
    pub containers: Vec<Container>,
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

    fn safename(&self) -> &String {
        &self.manifest.safename
    }
}

/// Holds the name and raw log files of a container within a pod.
#[derive(Debug, Clone)]
pub struct Container {
    pub name: String,
    pub current_log: String,
}

impl Container {
    pub fn safename(&self) -> String {
        render_safename(&self.name.as_str())
    }
}
