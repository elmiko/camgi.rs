// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod machine;
pub mod machineset;
pub mod node;

pub use crate::resources::machine::Machine;
pub use crate::resources::machineset::MachineSet;
pub use crate::resources::node::Node;
use crate::Manifest;

pub trait Resource {
    fn from(manifest: Manifest) -> Self;
    fn name(&self) -> &String;
    fn raw(&self) -> &String;
    fn safename(&self) -> &String;
    fn is_error(&self) -> bool;
}
