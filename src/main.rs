mod hypergraph;
mod rules;
mod simulation;

use rules::simple_rules::AddNodeRule;
use simulation::Simulation;

fn main() {
    let add_node_rule = AddNodeRule;
    let mut sim = Simulation::new(vec![&add_node_rule]);
    sim.run(5);
}
