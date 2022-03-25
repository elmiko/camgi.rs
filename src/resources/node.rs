use crate::prelude::*;

pub struct Node {
    raw: String,
    name: String,
    safename: String,
}

impl Node {
    pub fn from(manifest: Manifest) -> Node {
        Node {
            raw: manifest.as_raw(),
            name: manifest.name(),
            safename: manifest.safename(),
        }
    }
}

impl Resource for Node {
    fn name(&self) -> &String {
        &self.name
    }

    fn raw(&self) -> &String {
        &self.raw
    }

    fn safename(&self) -> &String {
        &self.safename
    }
}
