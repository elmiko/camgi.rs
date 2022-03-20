use crate::prelude::*;

pub struct Node {
    pub raw: String,
    pub name: String,
    pub safename: String,
}

impl Node {
    pub fn from(manifest: Manifest) -> Node {
        let name = match manifest.as_yaml()["metadata"]["name"].as_str() {
            Some(n) => String::from(n),
            None => String::from("Unknown"),
        };
        let safename = name.replace('.', "-");
        Node {
            raw: manifest.as_raw(),
            name,
            safename,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_node_name() {
        let expected = String::from("ip-10-0-0-1.control.plane");
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/core/nodes/ip-10-0-0-1.control.plane.yaml"
        )).unwrap();
        let node = Node::from(manifest);
        assert_eq!(node.name, expected)
    }

    #[test]
    fn test_node_raw() {
        let expected = include_str!(
            "../../testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/core/nodes/ip-10-0-0-1.control.plane.yaml");
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/core/nodes/ip-10-0-0-1.control.plane.yaml"
        )).unwrap();
        let node = Node::from(manifest);
        assert_eq!(node.raw, expected)
    }

    #[test]
    fn test_node_safename() {
        let expected = String::from("ip-10-0-0-1-control-plane");
        let manifest = Manifest::from(PathBuf::from(
            "testdata/must-gather-valid/sample-openshift-release/cluster-scoped-resources/core/nodes/ip-10-0-0-1.control.plane.yaml"
        )).unwrap();
        let node = Node::from(manifest);
        assert_eq!(node.safename, expected)
    }
}
