use field_derive::*;
use std::collections::BTreeMap;
use zoon::{Deserialize, Serialize};

mod exec_noop;
mod execnopmatch;
mod file;
mod header_regexp;
mod maxmind_geolocation;
mod path_regexp;
mod remote_host;
mod remote_ip;
mod vars_regexp;

#[ignore_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Field)]
#[serde(crate = "serde", default)]
pub(super) struct Val {
    exec_noop: Mutable<Option<exec_noop::Val>>,
    execnopmatch: Mutable<Option<execnopmatch::Val>>,
    expression: Mutable<Option<String>>,
    header: Mutable<Option<BTreeMap<String, crate::MSVec>>>,
    header_regexp: Mutable<Option<BTreeMap<String, header_regexp::Val>>>,
    host: Mutable<Option<Vec<String>>>,
    method: Mutable<Option<Vec<String>>>,
    not: Mutable<Option<Vec<Val>>>,
    path: Mutable<Option<Vec<String>>>,
    path_regexp: Mutable<Option<path_regexp::Val>>,
    protocol: Mutable<Option<String>>,
    query: Mutable<Option<BTreeMap<String, crate::MSVec>>>,
    remote_ip: Mutable<Option<remote_ip::Val>>,
    vars: Mutable<Option<BTreeMap<String, crate::MString>>>,
    vars_regexp: Mutable<Option<BTreeMap<String, vars_regexp::Val>>>,
    file: Mutable<Option<file::Val>>,
    remote_host: Mutable<Option<remote_host::Val>>,
    maxmind_geolocation: Mutable<Option<maxmind_geolocation::Val>>,
}

fn expression_default() -> String {
    Default::default()
}

fn host_default() -> String {
    Default::default()
}

fn method_default() -> String {
    "Get".to_owned()
}

fn path_default() -> String {
    Default::default()
}

fn protocol_default() -> String {
    Default::default()
}
