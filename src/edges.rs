use crate::{EdgeWeight, RouteConfig};
use hashbrown::HashMap;
use osm_models::{node::Node, ElementID};

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
        if let Some(c) = self.items.get_mut(&node) {
            c
        } else {
            self.items
                .insert(node, HashMap::<ElementID, EdgeWeight>::new());
            self.items.get_mut(&node).unwrap()
        }
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
}
