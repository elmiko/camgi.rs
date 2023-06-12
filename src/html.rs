// Copyright (C) 2022 Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::prelude::*;
use crate::resources;
use crate::resources::{Pod, Resource};
use html_builder::*;
use std::fmt::Write;
use uuid::Uuid;

pub struct Html {
    buffer: Buffer,
}

impl Html {
    pub fn from(mustgather: MustGather) -> Result<Html> {
        let mut buffer = Buffer::new();
        let mut html = buffer.html().attr("lang='en'");

        add_head(&mut html, &mustgather)?;
        add_body(&mut html, &mustgather)?;

        Ok(Html { buffer })
    }
}

impl Html {
    pub fn render(self) -> String {
        self.buffer.finish()
    }
}

// helper functions in alphabetical order

fn add_accordion_section(
    parent: &mut Node,
    kind: &str,
    resources: &Vec<impl Resource>,
) -> Result<()> {
    parent.h1().write_str(kind)?;
    let mut div = parent
        .div()
        .attr("class=\"accordion\"")
        .attr(format!("id=\"{}-accordion\"", kind.to_lowercase()).as_str());
    for res in resources {
        // use a UUID for the resource CSS id to avoid resource with similar names
        let resuuid = Uuid::new_v4();
        let mut itemdiv = div.div().attr("class=\"accordion-item\"");
        let buttonclass = match (res.is_warning(), res.is_error()) {
            (true, false) => " bg-warning text-white",
            (_, true) => " bg-danger text-white",
            _ => "",
        };
        itemdiv
            .h2()
            .attr("class=\"accordion-header\"")
            .attr(format!("id=\"heading-{}\"", &resuuid.hyphenated()).as_str())
            .button()
            .attr(format!("class=\"accordion-button collapsed p-2{}\"", buttonclass).as_str())
            .attr("type=\"button\"")
            .attr("data-bs-toggle=\"collapse\"")
            .attr(format!("data-bs-target=\"#collapse-{}\"", &resuuid.hyphenated()).as_str())
            .attr("aria-exapnded=\"false\"")
            .attr(format!("aria-controls=\"collapse-{}\"", &resuuid.hyphenated()).as_str())
            .write_str(&res.name())?;
        itemdiv
            .div()
            .attr(format!("id=\"collapse-{}\"", &resuuid.hyphenated()).as_str())
            .attr("class=\"accordion-collapse collapse\"")
            .attr(format!("aria-labelledby=\"heading-{}\"", &resuuid.hyphenated()).as_str())
            .attr(format!("data-bs-parents=\"{}-accordion\"", kind.to_lowercase()).as_str())
            .div()
            .attr("class=\"accordion-body fs-6\"")
            .pre()
            .write_str(&res.raw())?;
    }

    Ok(())
}

fn add_autoscaling_data(parent: &mut Node, mustgather: &MustGather) -> Result<()> {
    let mut data = parent.data().attr("id=\"autoscaling-data\"");

    add_accordion_section(
        &mut data,
        "ClusterAutoscalers",
        &mustgather.clusterautoscalers,
    )?;

    data.div().attr("class=\"p-2\"");

    add_accordion_section(
        &mut data,
        "MachineAutoscalers",
        &mustgather.machineautoscalers,
    )?;

    Ok(())
}

fn add_body(parent: &mut Node, mustgather: &MustGather) -> Result<()> {
    let mut body = parent.body().attr("class=\"bg-secondary\"");

    // main div
    let mut app = body
        .div()
        .attr("id=\"app\"")
        .attr("class=\"container-fluid\"");
    let mut row = app.div().attr("class=\"row mt-2\"");

    // nav list
    let mut nav = row.div().attr("class=\"col-2\"").attr("id=\"nav-col\"");
    let mut navlist = nav.div().attr("class=\"list-group\"");

    // summary nav entry
    navlist
        .button()
        .attr("v-on:click=\"changeContent('summary')\"")
        .attr("class=\"list-group-item list-group-item-action\"")
        .write_str("Summary")?;

    // cluster info sections
    add_navlist_entry(
        &mut navlist,
        "Cluster Operators",
        &mustgather.clusteroperators,
    )?;

    // nav entries for component sections
    add_navlist_entry(&mut navlist, "Machine API", &mustgather.mapipods)?;
    add_navlist_entry(&mut navlist, "Machine Config", &mustgather.mcopods)?;
    add_navlist_entry(&mut navlist, "CCM Operator", &mustgather.ccmopods)?;
    add_navlist_entry(&mut navlist, "CCMs", &mustgather.ccmpods)?;

    // nav entries for resources
    add_navlist_entry(&mut navlist, "Autoscaling", &mustgather.clusterautoscalers)?;
    add_navlist_entry(&mut navlist, "MachineSets", &mustgather.machinesets)?;
    add_navlist_entry(&mut navlist, "CPMSs", &mustgather.controlplanemachinesets)?;
    add_navlist_entry(&mut navlist, "Machines", &mustgather.machines)?;
    add_navlist_entry(&mut navlist, "BareMetalHosts", &mustgather.baremetalhosts)?;
    add_navlist_entry(&mut navlist, "Nodes", &mustgather.nodes)?;
    add_navlist_entry(&mut navlist, "CSRs", &mustgather.csrs)?;

    // github link should go last
    navlist
        .a()
        .attr("href=\"https://github.com/elmiko/camgi.rs\"")
        .attr("class=\"list-group-item list-group-item-action text-center\"")
        .attr("target=\"_blank\"")
        .img()
        .attr("src=\"https://github.com/favicon.ico\"")
        .attr("alt=\"GitHub logo\"")
        .attr("title=\"Found a bug or issue? Visit this project's git repo.\"");

    // main-content div
    let mut content = row.div().attr("class=\"col-10 bg-white rounded\"");
    content
        .div()
        .attr("id=\"main-content\"")
        .attr("class=\"overflow-auto\"")
        .span()
        .attr("v-html=\"content\"");

    // add data sections
    // data sections are used by the nav list and vue app to change the content
    // in the div#main-content element.
    add_summary_data(&mut body, &mustgather)?;
    add_resource_data(&mut body, "Cluster Operators", &mustgather.clusteroperators)?;
    add_machine_api_data(&mut body, &mustgather)?;
    add_machine_config_data(&mut body, &mustgather)?;
    add_ccmo_data(&mut body, &mustgather)?;
    add_ccms_data(&mut body, &mustgather)?;
    add_autoscaling_data(&mut body, &mustgather)?;
    add_resource_data(&mut body, "MachineSets", &mustgather.machinesets)?;
    add_resource_data(&mut body, "Machines", &mustgather.machines)?;
    add_resource_data(&mut body, "Nodes", &mustgather.nodes)?;
    add_resource_data(&mut body, "BareMetalHosts", &mustgather.baremetalhosts)?;
    add_resource_data(&mut body, "CPMSs", &mustgather.controlplanemachinesets)?;
    add_resource_data(&mut body, "CSRs", &mustgather.csrs)?;

    // scripts
    body.script()
        .attr("src=\"https://cdn.jsdelivr.net/npm/bootstrap@5.0.0-beta3/dist/js/bootstrap.bundle.min.js\"")
        .attr("integrity=\"sha384-JEW9xMcG8R+pH31jmWH6WWP0WintQrMb4s7ZOdauHnUtxwoG2vI5DkLtS3qm9Ekf\"")
        .attr("crossorigin=\"anonymous\"");
    body.script()
        .attr("src=\"https://cdn.jsdelivr.net/npm/vue@2/dist/vue.js\"");
    body.script()
        .write_str(include_str!("files/index_script.js"))?;

    Ok(())
}

fn add_ccmo_data(parent: &mut Node, mustgather: &MustGather) -> Result<()> {
    let mut data = parent.data().attr("id=\"ccm_operator-data\"");

    data.h1()
        .write_str("Cloud Controller Manager Operator Pods")?;

    add_pod_accordions(&mut data, &mustgather.ccmopods)?;

    Ok(())
}

fn add_ccms_data(parent: &mut Node, mustgather: &MustGather) -> Result<()> {
    let mut data = parent.data().attr("id=\"ccms-data\"");

    data.h1().write_str("Cloud Controller Manager Pods")?;

    add_pod_accordions(&mut data, &mustgather.ccmpods)?;

    Ok(())
}

fn add_head(parent: &mut Node, mustgather: &MustGather) -> Result<()> {
    let mut head = parent.head();
    head.title()
        .write_str(format!("camgi-{}", mustgather.title).as_str())?;
    head.meta().attr("charset='utf-8'");
    head.link()
        .attr("href=\"https://cdn.jsdelivr.net/npm/bootstrap@5.0.0-beta3/dist/css/bootstrap.min.css\"")
        .attr("rel=\"stylesheet\"")
        .attr("integrity=\"sha384-eOJMYsd53ii+scO/bJGFsiCZc+5NDVN2yr8+0RDqr0Ql0h+rP48ckxlpbzKgwra6\"")
        .attr("crossorigin=\"anonymous\"");
    head.style()
        .write_str(include_str!("files/index_style.css"))?;
    Ok(())
}

fn add_machine_api_data(parent: &mut Node, mustgather: &MustGather) -> Result<()> {
    let mut data = parent.data().attr("id=\"machine_api-data\"");

    data.h1().write_str("Machine API Pods")?;

    add_pod_accordions(&mut data, &mustgather.mapipods)?;

    Ok(())
}

fn add_machine_config_data(parent: &mut Node, mustgather: &MustGather) -> Result<()> {
    let mut data = parent.data().attr("id=\"machine_config-data\"");

    data.h1().write_str("Machine Config Operator Pods")?;

    add_pod_accordions(&mut data, &mustgather.mcopods)?;

    Ok(())
}

fn add_navlist_entry(parent: &mut Node, title: &str, resources: &Vec<impl Resource>) -> Result<()> {
    let mut aclass = "class=\"list-group-item list-group-item-action\"";
    let mut clickattr = format!(
        "v-on:click=\"changeContent('{}')\"",
        title.replace(' ', "_").to_lowercase()
    );
    if resources.is_empty() {
        aclass = "class=\"list-group-item list-group-item-action disabled\"";
        clickattr = String::new();
    }
    let mut button = parent.button().attr(aclass).attr(clickattr.as_str());
    button.write_str(title)?;

    if !resources.is_empty() {
        let errors = resources.iter().filter(|r| r.is_error()).count();
        if errors > 0 {
            button
                .span()
                .attr("class=\"badge bg-danger float-right\"")
                .write_str(format!("{}", errors).as_str())?;
        }

        let warnings = resources.iter().filter(|r| r.is_warning()).count();
        if warnings > 0 {
            button
                .span()
                .attr("class=\"badge bg-warning float-right\"")
                .write_str(format!("{}", warnings).as_str())?;
        }
    }
    Ok(())
}

fn add_pod_accordions(parent: &mut Node, pods: &Vec<Pod>) -> Result<()> {
    for pod in pods {
        let poduuid = Uuid::new_v4();
        let mut div = parent
            .div()
            .attr("class=\"accordion\"")
            .attr(format!("id=\"accordion-{}\"", &poduuid.hyphenated()).as_str());

        let mut itemdiv = div.div().attr("class=\"accordion-item\"");
        let buttonclass = match (pod.is_warning(), pod.is_error()) {
            (true, false) => " bg-warning text-white",
            (_, true) => " bg-danger text-white",
            _ => "",
        };
        itemdiv
            .h2()
            .attr("class=\"accordion-header\"")
            .attr(format!("id=\"heading-{}\"", &poduuid.hyphenated()).as_str())
            .button()
            .attr(format!("class=\"accordion-button collapsed p-2{}\"", buttonclass).as_str())
            .attr("type=\"button\"")
            .attr("data-bs-toggle=\"collapse\"")
            .attr(format!("data-bs-target=\"#collapse-{}\"", &poduuid.hyphenated()).as_str())
            .attr("aria-exapnded=\"false\"")
            .attr(format!("aria-controls=\"collapse-{}\"", &poduuid.hyphenated()).as_str())
            .write_str(&pod.name())?;
        itemdiv
            .div()
            .attr(format!("id=\"collapse-{}\"", &poduuid.hyphenated()).as_str())
            .attr("class=\"accordion-collapse collapse\"")
            .attr(format!("aria-labelledby=\"heading-{}\"", &poduuid.hyphenated()).as_str())
            .attr(format!("data-bs-parents=\"accordion-{}\"", &poduuid.hyphenated(),).as_str())
            .div()
            .attr("class=\"accordion-body fs-6\"")
            .pre()
            .write_str(&pod.raw())?;
        for container in &pod.containers {
            let containeruuid = Uuid::new_v4();
            let mut itemdiv = div.div().attr("class=\"accordion-item\"");
            let mut containerdiv = itemdiv
                .div()
                .attr("class=\"accordion\"")
                .attr(format!("id=\"logs-accordion-{}\"", &containeruuid.hyphenated(),).as_str());
            containerdiv
                .h2()
                .attr("class=\"accordion-header\"")
                .attr(format!("id=\"heading-logs-{}\"", &containeruuid.hyphenated(),).as_str())
                .button()
                .attr("class=\"accordion-button collapsed p-2 ps-4 bg-light\"")
                .attr("type=\"button\"")
                .attr("data-bs-toggle=\"collapse\"")
                .attr(
                    format!(
                        "data-bs-target=\"#collapse-logs-{}\"",
                        &containeruuid.hyphenated(),
                    )
                    .as_str(),
                )
                .attr("aria-exapnded=\"false\"")
                .attr(
                    format!(
                        "aria-controls=\"collapse-logs-{}\"",
                        &containeruuid.hyphenated(),
                    )
                    .as_str(),
                )
                .write_str(format!("{} logs", &container.name).as_str())?;
            containerdiv
                .div()
                .attr(format!("id=\"collapse-logs-{}\"", &containeruuid.hyphenated(),).as_str())
                .attr("class=\"accordion-collapse collapse\"")
                .attr(
                    format!(
                        "aria-labelledby=\"heading-logs-{}\"",
                        &containeruuid.hyphenated(),
                    )
                    .as_str(),
                )
                .attr(
                    format!(
                        "data-bs-parents=\"logs-accordion-{}\"",
                        &containeruuid.hyphenated(),
                    )
                    .as_str(),
                )
                .div()
                .attr("class=\"accordion-body fs-6\"")
                .pre()
                .write_str(&container.current_log)?;
        }
    }

    Ok(())
}

fn add_resource_data(parent: &mut Node, kind: &str, resources: &Vec<impl Resource>) -> Result<()> {
    let mut data = parent
        .data()
        .attr(format!("id=\"{}-data\"", kind.replace(' ', "_").to_lowercase()).as_str());
    add_accordion_section(&mut data, &kind, &resources)
}

fn add_summary_data(parent: &mut Node, mustgather: &MustGather) -> Result<()> {
    let mut data = parent.data().attr("id=\"summary-data\"");
    data.h1().write_str("Summary")?;
    data.hr();
    let mut dl = data.dl();

    dl.dt()
        .attr("class=\"text-light bg-secondary ps-1 mb-1\"")
        .write_str("Cluster")?;
    let mut dd = dl.dd();
    add_table(
        &mut dd,
        Vec::new(),
        vec![vec!["OpenShift Version", mustgather.version.as_str()]],
    )?;

    add_summary_data_machinesets_section(&mut dl, &mustgather)?;
    add_summary_data_machines_section(&mut dl, &mustgather)?;
    add_summary_data_nodes_section(&mut dl, &mustgather)?;

    Ok(())
}

fn add_summary_data_machines_section(parent: &mut Node, mustgather: &MustGather) -> Result<()> {
    parent
        .dt()
        .attr("class=\"text-light bg-secondary ps-1 mb-1\"")
        .write_str(format!("{} Machines", mustgather.machines.len()).as_str())?;
    let mut dd = parent.dd();
    let notrunning: Vec<resources::Machine> = mustgather
        .machines
        .iter()
        .filter(|m| m.is_error())
        .cloned()
        .collect();
    if !notrunning.is_empty() {
        dd.span()
            .attr("class=\"badge bg-danger\"")
            .write_str(format!("{}", notrunning.len()).as_str())?;
        dd.write_str("Machines not in Running phase")?;
        add_table(
            &mut dd,
            Vec::new(),
            notrunning.iter().map(|m| vec![m.name().as_str()]).collect(),
        )?;
    } else {
        dd.write_str("All Machines in Running phase")?;
    }

    Ok(())
}

fn add_summary_data_machinesets_section(parent: &mut Node, mustgather: &MustGather) -> Result<()> {
    parent
        .dt()
        .attr("class=\"text-light bg-secondary ps-1 mb-1\"")
        .write_str(format!("{} MachineSets", mustgather.machinesets.len()).as_str())?;
    let mut dd = parent.dd();
    let autoscaling: Vec<resources::MachineSet> = mustgather
        .machinesets
        .iter()
        .filter(|m| m.is_autoscaling())
        .cloned()
        .collect();
    if !autoscaling.is_empty() {
        dd.span()
            .attr("class=\"badge bg-secondary\"")
            .write_str(format!("{}", autoscaling.len()).as_str())?;
        dd.write_str("participating in autoscaling")?;
        add_table(
            &mut dd,
            vec!["Name", "Replicas"],
            autoscaling
                .iter()
                .map(|m| vec![m.name().as_str(), m.replicas()])
                .collect(),
        )?;
    } else {
        dd.write_str("None participating in autoscaling")?;
    }

    Ok(())
}

fn add_summary_data_nodes_section(parent: &mut Node, mustgather: &MustGather) -> Result<()> {
    // Nodes section
    parent
        .dt()
        .attr("class=\"text-light bg-secondary ps-1 mb-1\"")
        .write_str(format!("{} Nodes", mustgather.nodes.len()).as_str())?;
    let mut dd = parent.dd();
    let notready: Vec<resources::Node> = mustgather
        .nodes
        .iter()
        .filter(|n| n.is_error())
        .cloned()
        .collect();
    if !notready.is_empty() {
        dd.span()
            .attr("class=\"badge bg-danger\"")
            .write_str(format!("{}", notready.len()).as_str())?;
        dd.write_str("Nodes do not have a Ready condition")?;
        add_table(
            &mut dd,
            Vec::new(),
            notready.iter().map(|n| vec![n.name().as_str()]).collect(),
        )?;
    } else {
        dd.write_str("All nodes ready")?;
    }

    Ok(())
}

fn add_table(parent: &mut Node, head: Vec<&str>, body: Vec<Vec<&str>>) -> Result<()> {
    let mut table = parent
        .table()
        .attr("class=\"table table-sm table-striped font-monospace\"");

    if !head.is_empty() {
        let mut thead = table.thead();
        let mut tr = thead.tr();
        for (i, item) in head.iter().enumerate() {
            let t = if i == 0 { tr.th() } else { tr.td() };
            t.attr("scope=\"col\"").write_str(item)?;
        }
    }

    let mut tbody = table.tbody();

    for (_i, item) in body.iter().enumerate() {
        let mut tr = tbody.tr();
        for (ii, iitem) in item.iter().enumerate() {
            let t = if ii == 0 { tr.th() } else { tr.td() };
            t.attr("scope=\"col\"").write_str(iitem)?;
        }
    }

    Ok(())
}
