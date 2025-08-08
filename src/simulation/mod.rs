use crate::hypergraph::structure::HyperGraph;
use crate::rules::Rule;

pub struct Simulation<'a> {
    pub graph: HyperGraph,
    pub rules: Vec<&'a dyn Rule>,
}

impl<'a> Simulation<'a> {
    pub fn new(rules: Vec<&'a dyn Rule>) -> Self {
        Self {
            graph: HyperGraph::new(),
            rules,
        }
    }
    pub fn run(&mut self, steps: usize) {
        for step in 0..steps {
            println!("=== Step {} ===", step + 1);
            for rule in &self.rules {
                let changed = rule.apply(&mut self.graph);
                if changed {
                    println!("Rule applied: '{}'", rule.name());
                }
            }
            println!("Current graph state: {:?}", self.graph);
        }
    }
}
