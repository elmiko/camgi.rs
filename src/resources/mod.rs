// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod baremetalhost;
pub mod certificatesigningrequest;
pub mod clusterautoscaler;
pub mod clusteroperator;
pub mod controlplanemachineset;
pub mod machine;
pub mod machineautoscaler;
pub mod machineset;
pub mod node;
pub mod pod;

pub use crate::resources::baremetalhost::BareMetalHost;
pub use crate::resources::certificatesigningrequest::CertificateSigningRequest;
pub use crate::resources::clusterautoscaler::ClusterAutoscaler;
pub use crate::resources::clusteroperator::ClusterOperator;
pub use crate::resources::controlplanemachineset::ControlPlaneMachineSet;
pub use crate::resources::machine::Machine;
pub use crate::resources::machineautoscaler::MachineAutoscaler;
pub use crate::resources::machineset::MachineSet;
pub use crate::resources::node::Node;
pub use crate::resources::pod::Container;
pub use crate::resources::pod::Pod;
use crate::Manifest;

pub trait Resource {
    fn from(manifest: Manifest) -> Self;
    fn name(&self) -> &String;
    fn raw(&self) -> &String;

    fn is_error(&self) -> bool {
        false
    }

    fn is_warning(&self) -> bool {
        false
    }
}
