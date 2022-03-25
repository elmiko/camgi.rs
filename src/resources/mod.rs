pub mod node;

pub use crate::resources::node::Node;

pub trait Resource {
    fn name(&self) -> &String;
    fn raw(&self) -> &String;
    fn safename(&self) -> &String;
}
