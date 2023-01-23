// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::{GroupKindResource, Resource, ResourceScope};

#[derive(Debug, Clone)]
pub struct ClusterAutoscaler {
    manifest: Manifest,
}

#[allow(non_upper_case_globals)]
impl GroupKindResource for ClusterAutoscaler {
    const group: &'static str = "autoscaling.openshift.io";
    const kind: &'static str = "clusterautoscaler";
    const scope: ResourceScope = ResourceScope::Cluster;
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
}
