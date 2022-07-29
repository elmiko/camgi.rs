// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::Resource;

#[derive(Debug, Clone)]
pub struct CertificateSigningRequest {
    manifest: Manifest,
    pending: bool,
    denied: bool,
    failed: bool,
}

impl Resource for CertificateSigningRequest {
    fn from(manifest: Manifest) -> CertificateSigningRequest {
        new_from_manifest(manifest)
    }

    fn name(&self) -> &String {
        &self.manifest.name
    }

    fn raw(&self) -> &String {
        &self.manifest.as_raw()
    }

    fn is_error(&self) -> bool {
        self.denied || self.failed
    }

    fn is_warning(&self) -> bool {
        self.pending
    }
}

fn is_status_empty(manifest: &Manifest) -> bool {
    if !manifest.as_yaml()["status"].is_null() {
        if let Some(status) = manifest.as_yaml()["status"].as_hash() {
            if status.is_empty() {
                return true;
            }
        }
    }
    false
}

fn new_from_manifest(manifest: Manifest) -> CertificateSigningRequest {
    let pending = is_status_empty(&manifest);
    let denied = manifest.has_condition("Denied");
    let failed = manifest.has_condition("Failed");
    CertificateSigningRequest {
        manifest,
        pending,
        denied,
        failed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_status_empty_false() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/certificates.k8s.io/certificatesigningrequests/csr-approved.yaml"
        )).unwrap();
        assert_eq!(is_status_empty(&manifest), false);
    }

    #[test]
    fn test_status_empty_true() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/certificates.k8s.io/certificatesigningrequests/csr-pending.yaml"
        )).unwrap();
        assert_eq!(is_status_empty(&manifest), true);
    }

    #[test]
    fn test_is_warning_true() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/certificates.k8s.io/certificatesigningrequests/csr-pending.yaml"
        )).unwrap();
        let csr = new_from_manifest(manifest);
        assert_eq!(csr.is_warning(), true);
    }

    #[test]
    fn test_is_warning_false() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/certificates.k8s.io/certificatesigningrequests/csr-approved.yaml"
        )).unwrap();
        let csr = new_from_manifest(manifest);
        assert_eq!(csr.is_warning(), false);
    }

    #[test]
    fn test_is_error_false() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/certificates.k8s.io/certificatesigningrequests/csr-approved.yaml"
        )).unwrap();
        let csr = new_from_manifest(manifest);
        assert_eq!(csr.is_error(), false);
    }

    #[test]
    fn test_is_error_true_denied() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/certificates.k8s.io/certificatesigningrequests/csr-denied.yaml"
        )).unwrap();
        let csr = new_from_manifest(manifest);
        assert_eq!(csr.is_error(), true);
    }

    #[test]
    fn test_is_error_true_failed() {
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/certificates.k8s.io/certificatesigningrequests/csr-failed.yaml"
        )).unwrap();
        let csr = new_from_manifest(manifest);
        assert_eq!(csr.is_error(), true);
    }
}
