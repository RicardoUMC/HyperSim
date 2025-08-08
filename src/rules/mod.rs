pub mod simple_rules;
use crate::hypergraph::structure::HyperGraph;

pub trait Rule {
    fn apply(&self, graph: &mut HyperGraph) -> bool;
    fn name(&self) -> &'static str;
}
