use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub struct MustGather {
    pub path: PathBuf,
}

impl MustGather {
    pub fn title(&self) -> String {
        String::from(self.path.file_name().unwrap().to_str().unwrap())
    }

    /// Build a path to a resource, does not guarantee that it exists.
    fn _build_manifest_path(
        &self,
        name: &str,
        namespace: &str,
        kind: &str,
        group: &str,
    ) -> PathBuf {
        let mut manifestpath = self.path.clone();

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
}

pub fn build_mustgather(path: String) -> Result<MustGather> {
    let path = find_must_gather_root(path)?;

    Ok(MustGather { path })
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
fn find_must_gather_root(path: String) -> Result<PathBuf> {
    let orig = PathBuf::from(&path);
    let vpath: PathBuf = [String::from(&path), String::from("version")]
        .iter()
        .collect();
    let npath: PathBuf = [String::from(&path), String::from("namespaces")]
        .iter()
        .collect();
    let csrpath: PathBuf = [
        String::from(&path),
        String::from("cluster-scoped-resources"),
    ]
    .iter()
    .collect();

    if vpath.is_file() || (npath.is_dir() && csrpath.is_dir()) {
        return Ok(orig.canonicalize().unwrap());
    }

    let directories: Vec<PathBuf> = fs::read_dir(orig)
        .unwrap()
        .into_iter()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .filter(|r| r.is_dir())
        .collect();

    if directories.len() == 1 {
        find_must_gather_root(String::from(directories[0].to_str().unwrap()))
    } else {
        Err(anyhow::anyhow!("Cannot determine root of must-gather"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_manifest_path_cluster_scoped() {
        let mg = MustGather {
            path: PathBuf::from("/foo"),
        };
        assert_eq!(
            mg._build_manifest_path("", "", "nodes", "core"),
            PathBuf::from("/foo/cluster-scoped-resources/core/nodes")
        )
    }

    #[test]
    fn test_build_manifest_path_cluster_scoped_named_resource() {
        let mg = MustGather {
            path: PathBuf::from("/foo"),
        };
        assert_eq!(
            mg._build_manifest_path("node1", "", "nodes", "core"),
            PathBuf::from("/foo/cluster-scoped-resources/core/nodes/node1.yaml")
        )
    }

    #[test]
    fn test_build_manifest_path_namespace_scoped() {
        let mg = MustGather {
            path: PathBuf::from("/foo"),
        };
        assert_eq!(
            mg._build_manifest_path(
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
        let mg = MustGather {
            path: PathBuf::from("/foo"),
        };
        assert_eq!(
            mg._build_manifest_path(
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
}
