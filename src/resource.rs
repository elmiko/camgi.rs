use std::path::PathBuf;

#[derive(Debug)]
pub struct Resource {
    pub name: String,
    pub yaml: String,
}

/// Build a Resource struct from a path to a manifest YAML.
pub fn build_resource(path: PathBuf) -> Option<Resource> {
    if !path.is_file() || path.is_dir() {
        return None;
    }

    None
}

/// Build a collection of Resources from a path to a directory.
pub fn build_resource_vec(path: PathBuf) -> Vec<Resource> {
    let resources: Vec<Resource> = Vec::new();

    resources
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_resource_none() {
        match build_resource(PathBuf::from(
            "testdata/must-gather-invalid/does-not-exist.yaml",
        )) {
            Some(_) => panic!("Unexpected return value"),
            None => (),
        }
    }

    #[test]
    fn test_build_resource_dir() {
        match build_resource(PathBuf::from(
            "testdata/must-gather-valid",
        )) {
            Some(_) => panic!("Unexpected return value"),
            None => (),
        }
    }
}
