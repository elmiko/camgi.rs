// Copyright (C) 2023 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::Resource;

#[derive(Debug, Clone)]
pub struct ClusterOperator {
    manifest: Manifest,
    available: bool,
    degraded: bool,
    upgradeable: bool,
    has_upgradeable: bool,
}

impl Resource for ClusterOperator {
    fn from(manifest: Manifest) -> ClusterOperator {
        let available = manifest.has_condition_status("Available", "True");
        let degraded = manifest.has_condition_status("Degraded", "True");
        let upgradeable = manifest.has_condition_status("Upgradeable", "True");
        let has_upgradeable = manifest.has_condition("Upgradeable");
        ClusterOperator {
            manifest,
            available,
            degraded,
            upgradeable,
            has_upgradeable,
        }
    }

    fn is_error(&self) -> bool {
        !self.available || self.degraded
    }

    fn is_warning(&self) -> bool {
        !self.upgradeable && self.has_upgradeable
    }

    fn name(&self) -> &String {
        &self.manifest.name
    }

    fn raw(&self) -> &String {
        &self.manifest.as_raw()
    }
}
