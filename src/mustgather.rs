// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources::*;
use std::fs;
use std::path::{Path, PathBuf};

pub struct MustGather {
    pub title: String,
    pub version: String,
    pub platformtype: String,
    pub clusteroperators: Vec<ClusterOperator>,
    pub machines: Vec<Machine>,
    pub machinesets: Vec<MachineSet>,
    pub nodes: Vec<Node>,
    pub csrs: Vec<CertificateSigningRequest>,
    pub clusterautoscalers: Vec<ClusterAutoscaler>,
    pub machineautoscalers: Vec<MachineAutoscaler>,
    pub baremetalhosts: Vec<BareMetalHost>,
    pub controlplanemachinesets: Vec<ControlPlaneMachineSet>,
    pub mapipods: Vec<Pod>,
    pub mcopods: Vec<Pod>,
    pub ccmopods: Vec<Pod>,
    pub ccmpods: Vec<Pod>,
}

impl MustGather {
    /// Build a MustGather from a path to a directory containing the root.
    pub fn from(path: impl AsRef<Path>) -> Result<MustGather> {
        Self::from_path(path.as_ref())
    }

    pub fn from_path(path: &Path) -> Result<MustGather> {
        let path = find_must_gather_root(path)?;
        let title = String::from(path.file_name().unwrap().to_str().unwrap());
        let version = get_cluster_version(&path);
        let platformtype = get_cluster_platform_type(&path);

        let manifestpath =
            build_manifest_path(&path, "", "", "clusteroperators", "config.openshift.io");
        let clusteroperators = get_resources::<ClusterOperator>(&manifestpath);

        let manifestpath = build_manifest_path(
            &path,
            "",
            "openshift-machine-api",
            "machines",
            "machine.openshift.io",
        );
        let machines = get_resources::<Machine>(&manifestpath);

        let manifestpath = build_manifest_path(
            &path,
            "",
            "openshift-machine-api",
            "machinesets",
            "machine.openshift.io",
        );
        let machinesets = get_resources::<MachineSet>(&manifestpath);

        let manifestpath = build_manifest_path(&path, "", "", "nodes", "core");
        let nodes = get_resources::<Node>(&manifestpath);

        let manifestpath = build_manifest_path(
            &path,
            "",
            "",
            "certificatesigningrequests",
            "certificates.k8s.io",
        );
        let csrs = get_resources::<CertificateSigningRequest>(&manifestpath);

        let manifestpath = build_manifest_path(
            &path,
            "",
            "",
            "clusterautoscalers",
            "autoscaling.openshift.io",
        );
        let clusterautoscalers = get_resources::<ClusterAutoscaler>(&manifestpath);

        let manifestpath = build_manifest_path(
            &path,
            "",
            "openshift-machine-api",
            "machineautoscalers",
            "autoscaling.openshift.io",
        );
        let machineautoscalers = get_resources::<MachineAutoscaler>(&manifestpath);

        let manifestpath = build_manifest_path(
            &path,
            "",
            "openshift-machine-api",
            "baremetalhosts",
            "metal3.io",
        );
        let baremetalhosts = get_resources::<BareMetalHost>(&manifestpath);

        let manifestpath = build_manifest_path(
            &path,
            "",
            "openshift-machine-api",
            "controlplanemachinesets",
            "machine.openshift.io",
        );
        let controlplanemachinesets = get_resources::<ControlPlaneMachineSet>(&manifestpath);

        let manifestpath = build_manifest_path(&path, "", "openshift-machine-api", "pods", "");
        let mapipods = get_pods(&manifestpath);

        let manifestpath =
            build_manifest_path(&path, "", "openshift-machine-config-operator", "pods", "");
        let mcopods = get_pods(&manifestpath);

        let manifestpath = build_manifest_path(
            &path,
            "",
            "openshift-cloud-controller-manager-operator",
            "pods",
            "",
        );
        let ccmopods = get_pods(&manifestpath);

        let manifestpath =
            build_manifest_path(&path, "", "openshift-cloud-controller-manager", "pods", "");
        let ccmpods = get_pods(&manifestpath);

        Ok(MustGather {
            title,
            version,
            platformtype,
            clusteroperators,
            machines,
            machinesets,
            nodes,
            csrs,
            clusterautoscalers,
            machineautoscalers,
            baremetalhosts,
            controlplanemachinesets,
            mapipods,
            mcopods,
            ccmopods,
            ccmpods,
        })
    }
}

/// Build a path to a resource, does not guarantee that it exists.
/// If a name is provided the path will include a yaml file. If the name is
/// an empty string the path will be to the directory containing the resource
/// manifest yaml files.
/// If the namespace is an emptry string then the path will be to cluster
/// scoped resources.
/// Example - finding node resources
/// build_manifest_path(mgroot, "", "", "nodes", "core")
/// Example - finding a specific machine
/// build_manifest_path(mgroot, "machine-name", "openshift-machine-api", "machines", "machine.openshift.io")
fn build_manifest_path(
    path: &Path,
    name: &str,
    namespace: &str,
    kind: &str,
    group: &str,
) -> PathBuf {
    let mut manifestpath = path.to_path_buf();

    if namespace.is_empty() {
        manifestpath.push("cluster-scoped-resources");
    } else {
        manifestpath.push("namespaces");
        manifestpath.push(namespace);
    }

    if !group.is_empty() {
        manifestpath.push(group);
    }

    manifestpath.push(kind);

    if !name.is_empty() {
        manifestpath.push(format!("{}.yaml", name));
    }

    manifestpath
}

/// Find the root of a must-gather directory structure given a path.
///
/// Finding the root of the must-gather is accomplished through the following steps:
/// 1. look for a `version` file in the current path, if it exists return current path.
/// 2. look for the directories `namespaces` and `cluster-scoped-resources` in the current path,
///    if they exist, return the current path.
/// 3. if there is a single subdirectory in the path, recursively run this function on it and
///    return the result.
/// 4. return an error
fn find_must_gather_root(path: &Path) -> Result<PathBuf> {
    let vpath = path.join("version");
    let npath = path.join("namespaces");
    let csrpath = path.join("cluster-scoped-resources");

    if vpath.is_file() || (npath.is_dir() && csrpath.is_dir()) {
        return Ok(path.canonicalize().unwrap());
    }

    let directory_entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return Err(anyhow::anyhow!("Unable to read directory {:?}", path)),
    };

    let directories: Vec<PathBuf> = directory_entries
        .into_iter()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .filter(|r| r.is_dir())
        .collect();

    if directories.len() == 1 {
        find_must_gather_root(&directories[0])
    } else {
        Err(anyhow::anyhow!("Cannot determine root of must-gather"))
    }
}

/// Get the platform type.
/// If unable to determine the platform, "Unknown" will be returned.
fn get_cluster_platform_type(path: &Path) -> String {
    let mut manifestpath =
        build_manifest_path(path, "", "", "infrastructures", "config.openshift.io");
    manifestpath.push("cluster.yaml");
    let version = match Manifest::from(manifestpath) {
        Ok(v) => v,
        Err(_) => return String::from("Unknown"),
    };
    match version.as_yaml()["status"]["platformStatus"]["type"].as_str() {
        Some(v) => String::from(v),
        None => String::from("Unknown"),
    }
}

/// Get the version string.
/// If unable to determine the version, "Unknown" will be returned.
fn get_cluster_version(path: &Path) -> String {
    let mut manifestpath =
        build_manifest_path(path, "", "", "clusterversions", "config.openshift.io");
    manifestpath.push("version.yaml");
    let version = match Manifest::from(manifestpath) {
        Ok(v) => v,
        Err(_) => return String::from("Unknown"),
    };
    match version.as_yaml()["status"]["desired"]["version"].as_str() {
        Some(v) => String::from(v),
        None => String::from("Unknown"),
    }
}

/// Get a pod from a path.
/// Will attempt to determine the pod name and containers, if it cannot
/// find the files or encounters an error, it will return None.
fn get_pod(pod_dir: &PathBuf) -> Option<Pod> {
    let manifest_yaml = match pod_dir.file_name() {
        Some(basename) => format!("{}.yaml", basename.to_str().unwrap_or("not_found")),
        None => return None,
    };

    let mut manifest_file = pod_dir.clone();
    manifest_file.push(manifest_yaml);
    let mut pod = Pod::new();
    if manifest_file.exists() {
        pod = match Manifest::from(manifest_file) {
            Ok(m) => <Pod as Resource>::from(m),
            Err(_) => return None,
        }
    }

    if let Ok(container_dirs) = fs::read_dir(&pod_dir) {
        // loop through container dirs
        for container_dir in container_dirs {
            let container_dir = container_dir.ok()?.path();
            //   build path to log file
            let container_name = match container_dir.file_name() {
                Some(basename) => basename.to_str().unwrap_or("not_found"),
                None => continue,
            };
            let mut current_log_filename = container_dir.clone();
            current_log_filename.push(&container_name);
            current_log_filename.push("logs");
            current_log_filename.push("current.log");
            if current_log_filename.exists() {
                //   if it exists open and read into a new string
                let raw = match fs::read_to_string(current_log_filename.as_path()) {
                    Ok(contents) => contents,
                    Err(_) => continue,
                };
                //   create a Container and add it to the Pod
                pod.push_container(Container {
                    name: container_name.to_string(),
                    current_log: raw,
                });
            }
        }
    }

    Some(pod)
}

/// Get all pods in a path.
/// Pod files within a must gather also include the associated logs for each
/// container. This function will find all the pod files within a path and
/// return the structured versions.
fn get_pods(path: &Path) -> Vec<Pod> {
    let mut pods = Vec::new();

    // each pod has a subdirectory with its name
    let pod_dirs = match fs::read_dir(&path) {
        Ok(entries) => entries,
        Err(_) => return pods,
    };
    let pod_dirs: Vec<PathBuf> = pod_dirs
        .into_iter()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .filter(|r| r.is_dir())
        .collect();

    for pod_dir in pod_dirs {
        let pod = match get_pod(&pod_dir) {
            Some(p) => p,
            None => continue,
        };
        pods.push(pod);
    }

    pods
}

/// Get all the resources of a given type.
/// If the resource path does not exist, will return an empty list.
fn get_resources<T: Resource>(path: &Path) -> Vec<T> {
    let mut resources = Vec::new();
    let files = match fs::read_dir(&path) {
        Ok(p) => p,
        Err(_) => return resources,
    };
    let yamlfiles: Vec<PathBuf> = files
        .into_iter()
        .filter(|m| m.is_ok())
        .map(|m| m.unwrap().path())
        .filter(|m| m.extension().unwrap() == "yaml")
        .collect();

    for path in yamlfiles {
        match Manifest::from(path) {
            Ok(m) => resources.push(T::from(m)),
            Err(_) => continue,
        }
    }
    resources
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_manifest_path_cluster_scoped() {
        assert_eq!(
            build_manifest_path(&PathBuf::from("/foo"), "", "", "nodes", "core"),
            PathBuf::from("/foo/cluster-scoped-resources/core/nodes")
        )
    }

    #[test]
    fn test_build_manifest_path_cluster_scoped_named_resource() {
        assert_eq!(
            build_manifest_path(&PathBuf::from("/foo"), "node1", "", "nodes", "core"),
            PathBuf::from("/foo/cluster-scoped-resources/core/nodes/node1.yaml")
        )
    }

    #[test]
    fn test_build_manifest_path_namespace_scoped() {
        assert_eq!(
            build_manifest_path(
                &PathBuf::from("/foo"),
                "",
                "openshift-machine-api",
                "machines",
                "machine.openshift.io"
            ),
            PathBuf::from("/foo/namespaces/openshift-machine-api/machine.openshift.io/machines")
        )
    }

    #[test]
    fn test_build_manifest_path_namespace_scoped_named_resource() {
        assert_eq!(
            build_manifest_path(
                &PathBuf::from("/foo"),
                "machine1",
                "openshift-machine-api",
                "machines",
                "machine.openshift.io"
            ),
            PathBuf::from(
                "/foo/namespaces/openshift-machine-api/machine.openshift.io/machines/machine1.yaml"
            )
        )
    }

    #[test]
    fn test_get_cluster_version() {
        assert_eq!(
            get_cluster_version(&PathBuf::from(
                "testdata/must-gather-valid/sample-openshift-release"
            )),
            "X.Y.Z-fake-test"
        )
    }

    #[test]
    fn test_get_pod_containers_count() {
        let path = PathBuf::from("testdata/must-gather-valid/sample-openshift-release/namespaces/openshift-machine-api/pods/machine-api-controllers-86c6c8f96d-ssrp8");
        let pod = get_pod(&path).unwrap();
        assert_eq!(pod.containers.len(), 7)
    }

    #[test]
    fn test_get_pods_success() {
        let path = PathBuf::from("testdata/must-gather-valid/sample-openshift-release");
        let manifestpath = build_manifest_path(&path, "", "openshift-machine-api", "pods", "");
        assert_eq!(get_pods(&manifestpath).len(), 4)
    }

    #[test]
    fn test_get_resources_success() {
        let path = PathBuf::from("testdata/must-gather-valid/sample-openshift-release");
        let manifestpath = build_manifest_path(&path, "", "", "nodes", "core");
        assert_eq!(get_resources::<Node>(&manifestpath).len(), 3)
    }

    #[test]
    fn test_get_resources_non_existant() {
        let path = PathBuf::from("testdata/must-gather-invalid/sample-openshift-release");
        let manifestpath = build_manifest_path(&path, "", "fake", "kind", "group");
        assert_eq!(get_resources::<Node>(&manifestpath).len(), 0)
    }
}
