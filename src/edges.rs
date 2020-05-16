use crate::{restriction::allow_travel_mode, EdgeWeight, RouteConfig};
use hashbrown::HashMap;
use lazy_static::*;
use osm_models::{
    tag::{self, Tagged},
    travel_by, way_type, ElementID, Node, Way,
};
use regex::Regex;

lazy_static! {
    static ref REVERSE: Regex = Regex::new("^(-1|reverse)$").unwrap();
    static ref FORWARD: Regex = Regex::new("^(yes|true|1)$").unwrap();
    static ref ONE_WAY: Regex =
        Regex::new("^(yes|true|1|-1|reverse)$").unwrap();
}

/// Weight (or below) indicating way is not usable
pub static CANNOT_USE: EdgeWeight = 0.0;
/// Weighted connection to another node
pub type Connection = HashMap<ElementID, EdgeWeight>;

pub struct Edges<'a> {
    /// Weights assigned to node-node connections based on `RouteConfig`
    items: HashMap<ElementID, Connection>,
    travel_mode: String,
    config: RouteConfig<'a>,
}

impl<'a> Edges<'a> {
    /// Number of edges
    fn len(&self) -> usize {
        self.items.len()
    }

    /// Weighted nodes (IDs) connected to the given node
    fn get(&self, node: ElementID) -> Option<&Connection> {
        self.items.get(&node)
    }

    fn get_or_insert(&mut self, node: ElementID) -> &mut Connection {
        self.items.entry(node).or_insert(HashMap::new())
    }

    fn has(&self, from: ElementID) -> bool {
        self.items.contains_key(&from)
    }

    /// Whether `from` node exists and is connected with `to` node
    fn has_connection(&self, from: ElementID, to: ElementID) -> bool {
        self.get(from).map_or(false, |m| m.contains_key(&to))
    }

    /// Weight for the connection between `from` node and `to` node.
    /// `CANNOT_USE` is returned if the nodes aren't connected.
    fn weight(&self, from: ElementID, to: ElementID) -> EdgeWeight {
        self.get(from)
            .map_or(CANNOT_USE, |m| *m.get(&to).unwrap_or(&CANNOT_USE))
    }

    /// Add connection weight between `from` and `to` node
    fn add(&mut self, from: &Node, to: &Node, weight: EdgeWeight) {
        let edge: &mut Connection = self.get_or_insert(from.id);
        edge.insert(to.id, weight);
    }

    fn ensure(&self, nodes: Vec<ElementID>) {
        for id in nodes {
            assert!(self.has(id));
        }
    }

    /// Execute method for each `toNode` that `nodeID` connects to
    fn each(
        &self,
        node: ElementID,
        cb: &dyn Fn(&EdgeWeight, &ElementID) -> (),
    ) {
        if let Some(connections) = self.get(node) {
            for (k, v) in connections {
                cb(v, k);
            }
        }
    }

    /// Map node edges to an array using given function. This may be used, for
    /// example, to map edges to route options.
    fn map<T>(
        &self,
        node: ElementID,
        cb: &dyn Fn(&EdgeWeight, &ElementID) -> T,
    ) -> Vec<T> {
        let mut out = Vec::new();

        if let Some(connections) = self.get(node) {
            for (k, v) in connections {
                out.push(cb(v, k));
            }
        }
        out
    }

    /// Add weighted edges from way and return routable nodes
    /// https://wiki.openstreetmap.org/wiki/Key:oneway
    fn from_way(&mut self, way: &Way) -> bool {
        let mut weight: EdgeWeight = CANNOT_USE;
        let mut one_way = way.get_tag(tag::ONE_WAY).unwrap_or("");
        let road_type = way.get_tag(tag::ROAD_TYPE);
        let rail_type = way.get_tag(tag::RAIL_TYPE);
        let junction = way.get_tag(tag::JUNCTION_TYPE);

        if one_way.is_empty()
            && (junction == Some("roundabout")
                || junction == Some("circular")
                || road_type == Some(way_type::FREEWAY))
        {
            // infer one-way for roundabouts and freeways
            one_way = tag::access::ALLOWED;
        }

        if self.travel_mode == travel_by::FOOT
            || (ONE_WAY.is_match(one_way)
                && way.get_tag(&format!(
                    "{}:{}",
                    tag::ONE_WAY,
                    self.travel_mode
                )) == Some(tag::access::NONE))
        {
            // disable one-way setting for foot traffic or explicit tag
            one_way = tag::access::NONE;
        }

        if let Some(t) = road_type {
            weight = *self.config.weights.get(t).unwrap_or(&CANNOT_USE);
        }

        if weight == CANNOT_USE {
            if let Some(t) = rail_type {
                // TODO: is this right? How can we arbitrarily switch to rail type?
                // see if there's another way
                weight = *self.config.weights.get(t).unwrap_or(&CANNOT_USE);
            }
        }

        if weight <= CANNOT_USE || allow_travel_mode(way, &self.config.can_use)
        {
            return false;
        }

        for (i, _) in way.nodes.iter().enumerate() {
            if i == 0 {
                continue;
            }
            let n1 = way.nodes.get(i - 1).unwrap();
            let n2 = way.nodes.get(i).unwrap();

            // foward travel is allowed from n1 to n2
            if !REVERSE.is_match(one_way) {
                self.add(n1, n2, weight);
            }
            // reverse travel is allowed from n2 to n1
            if !FORWARD.is_match(one_way) {
                self.add(n2, n1, weight);
            }
        }

        true
    }
}
