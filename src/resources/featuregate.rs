// Copyright (C) 2026 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::Resource;

#[derive(Debug, Clone)]
pub struct FeatureGate {
    manifest: Manifest,
    pub feature_set: String,
    pub feature_gates: Vec<FeatureGateDetails>,
    pub custom_enabled: Vec<String>,
    pub custom_disabled: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FeatureGateDetails {
    pub version: String,
    pub enabled: Vec<String>,
    pub disabled: Vec<String>,
}

impl Resource for FeatureGate {
    fn from(manifest: Manifest) -> FeatureGate {
        let yaml = manifest.as_yaml();
        let feature_set = match yaml["spec"]["featureSet"].as_str().unwrap_or("") {
            "" => String::from("Default"),
            value => String::from(value),
        };
        let feature_gates = yaml["status"]["featureGates"]
            .as_vec()
            .into_iter()
            .flatten()
            .map(|details| FeatureGateDetails {
                version: details["version"].as_str().unwrap_or("Unknown").to_string(),
                enabled: details["enabled"]
                    .as_vec()
                    .into_iter()
                    .flatten()
                    .filter_map(|gate| gate["name"].as_str().map(String::from))
                    .collect(),
                disabled: details["disabled"]
                    .as_vec()
                    .into_iter()
                    .flatten()
                    .filter_map(|gate| gate["name"].as_str().map(String::from))
                    .collect(),
            })
            .collect();
        let custom_enabled = yaml["spec"]["customNoUpgrade"]["enabled"]
            .as_vec()
            .into_iter()
            .flatten()
            .filter_map(|gate| gate.as_str().map(String::from))
            .collect();
        let custom_disabled = yaml["spec"]["customNoUpgrade"]["disabled"]
            .as_vec()
            .into_iter()
            .flatten()
            .filter_map(|gate| gate.as_str().map(String::from))
            .collect();
        FeatureGate {
            manifest,
            feature_set,
            feature_gates,
            custom_enabled,
            custom_disabled,
        }
    }

    fn name(&self) -> &String {
        &self.manifest.name
    }

    fn raw(&self) -> &String {
        self.manifest.as_raw()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_featuregate_from_manifest() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/config.openshift.io/featuregates/cluster.yaml",
        ))
        .unwrap();
        let gate = <FeatureGate as Resource>::from(manifest);
        assert_eq!(gate.name(), "cluster");
        assert_eq!(gate.feature_set, "DevPreviewNoUpgrade");
        assert_eq!(gate.feature_gates.len(), 1);
        assert_eq!(gate.feature_gates[0].version, "5.1.0-test");
        assert_eq!(gate.feature_gates[0].enabled, vec!["ClusterAPIOperator"]);
        assert_eq!(gate.feature_gates[0].disabled, vec!["ClusterAPIInstall"]);
    }

    #[test]
    fn test_featuregate_defaults() {
        let gate = <FeatureGate as Resource>::from(Manifest::new());
        assert_eq!(gate.feature_set, "Default");
        assert!(gate.feature_gates.is_empty());
        assert!(gate.custom_enabled.is_empty());
        assert!(gate.custom_disabled.is_empty());
    }
}
