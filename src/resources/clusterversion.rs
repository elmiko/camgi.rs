// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::{GroupKindResource, Resource, ResourceScope};

#[derive(Debug, Clone)]
pub struct ClusterVersion {
    manifest: Manifest,
    version: String,
}

#[allow(non_upper_case_globals)]
impl GroupKindResource for ClusterVersion {
    const group: &'static str = "config.openshift.io";
    const kind: &'static str = "clusterversion";
    const scope: ResourceScope = ResourceScope::ClusterSingleton;
}

impl Resource for ClusterVersion {
    fn from(manifest: Manifest) -> ClusterVersion {
        let version = parse_cluster_version(&manifest);
        ClusterVersion { manifest, version }
    }

    fn name(&self) -> &String {
        &self.manifest.name
    }

    fn raw(&self) -> &String {
        &self.manifest.as_raw()
    }
}

fn parse_cluster_version(manifest: &Manifest) -> String {
    match manifest.as_yaml()["status"]["desired"]["version"].as_str() {
        Some(v) => String::from(v),
        None => String::from("Unknown"),
    }
}
