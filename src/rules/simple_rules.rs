use super::Rule;
use crate::hypergraph::structure::HyperGraph;

pub struct AddNodeRule;

impl Rule for AddNodeRule {
    fn apply(&self, graph: &mut HyperGraph) -> bool {
        let node_id = graph.add_node();
        println!("AddNodeRule: Added node with ID: {}", node_id);
        true
    }
    fn name(&self) -> &'static str {
        "AddNodeRule"
    }
}
