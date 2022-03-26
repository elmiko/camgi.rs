// Copyright (C) 2022 Red Hat
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod machine;
pub mod node;

pub use crate::resources::machine::Machine;
pub use crate::resources::node::Node;

pub trait Resource {
    fn name(&self) -> &String;
    fn raw(&self) -> &String;
    fn safename(&self) -> &String;
    fn is_error(&self) -> bool;
}
