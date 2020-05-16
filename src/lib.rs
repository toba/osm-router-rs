#![allow(dead_code)]
#[macro_use]
extern crate maplit;

mod config;
mod edges;
mod restriction;

use std::collections::HashMap;

/// Weight given to an edge. Larger numbers indicate stronger preference.
pub type EdgeWeight = f32;

/// Permitted access types and road or rail type weighting that together define
/// routing preferences
pub struct RouteConfig<'a> {
    pub name: &'a str,
    /// Weights keyed to road types — larger numbers indicate stronger
    /// preference
    pub weights: HashMap<&'a str, EdgeWeight>,
    /// Usable access types ordered by specificity. The first item should be
    /// most general and the last most specific so when iterated, later types
    /// can override earlier ones.
    pub can_use: Vec<&'a str>,
}

/// Result of routing request
pub enum Status {
    /// Start and end nodes are not connected
    NoRoute,
    /// Found series of nodes connecting start to end
    Success,
    /// Maximum tries exceeded searching for connection between start and end
    /// nodes
    GaveUp,
}

pub type RouteMode<'a> = HashMap<&'a str, RouteConfig<'a>>;
