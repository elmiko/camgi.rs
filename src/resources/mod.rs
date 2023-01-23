// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod baremetalhost;
pub mod certificatesigningrequest;
pub mod clusterautoscaler;
pub mod clusterversion;
pub mod controlplanemachineset;
pub mod machine;
pub mod machineautoscaler;
pub mod machineset;
pub mod node;
pub mod pod;

pub use crate::resources::baremetalhost::BareMetalHost;
pub use crate::resources::certificatesigningrequest::CertificateSigningRequest;
pub use crate::resources::clusterautoscaler::ClusterAutoscaler;
pub use crate::resources::clusterversion::ClusterVersion;
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

pub enum ResourceScope {
    Namespaced,
    Cluster,
    ClusterSingleton,
}

#[allow(non_upper_case_globals)]
pub trait GroupKindResource: Resource {
    const group: &'static str;
    const kind: &'static str;
    const scope: ResourceScope;

    fn kind_plural() -> String {
        // TODO consider formatcp, maybe it's possible to move it into trait attrs as well
        // https://docs.rs/const_format/0.2.30/const_format/macro.formatcp.html
        format!("{}s", Self::kind)
    }
}
