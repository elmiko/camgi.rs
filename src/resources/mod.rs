// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod awscluster;
pub mod awsmachine;
pub mod awsmachinetemplate;
pub mod baremetalhost;
pub mod capicluster;
pub mod capimachine;
pub mod capimachinedeployment;
pub mod capimachineset;
pub mod certificatesigningrequest;
pub mod clusterautoscaler;
pub mod clusteroperator;
pub mod controlplanemachineset;
pub mod machine;
pub mod machineautoscaler;
pub mod machineconfig;
pub mod machineconfigpool;
pub mod machineconfiguration;
pub mod machineset;
pub mod node;
pub mod pod;

pub use crate::resources::awscluster::AWSCluster;
pub use crate::resources::awsmachine::AWSMachine;
pub use crate::resources::awsmachinetemplate::AWSMachineTemplate;
pub use crate::resources::baremetalhost::BareMetalHost;
pub use crate::resources::capicluster::CAPICluster;
pub use crate::resources::capimachine::CAPIMachine;
pub use crate::resources::capimachinedeployment::CAPIMachineDeployment;
pub use crate::resources::capimachineset::CAPIMachineSet;
pub use crate::resources::certificatesigningrequest::CertificateSigningRequest;
pub use crate::resources::clusterautoscaler::ClusterAutoscaler;
pub use crate::resources::clusteroperator::ClusterOperator;
pub use crate::resources::controlplanemachineset::ControlPlaneMachineSet;
pub use crate::resources::machine::Machine;
pub use crate::resources::machineautoscaler::MachineAutoscaler;
pub use crate::resources::machineconfig::MachineConfig;
pub use crate::resources::machineconfigpool::MachineConfigPool;
pub use crate::resources::machineconfiguration::MachineConfiguration;
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

    fn conditions(&self) -> Vec<String> {
        Vec::new()
    }
}
