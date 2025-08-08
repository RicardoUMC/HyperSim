pub type NodeId = usize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    pub id: NodeId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HyperEdge {
    pub id: usize,
    pub nodes: Vec<NodeId>,
}

#[derive(Debug, Default)]
struct HyperGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<HyperEdge>,
}

impl HyperGraph {
    pub fn new() -> Self {
        Hypergraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    pub fn add_node(&mut self) -> NodeId {
        let id = self.nodes.len();
        self.nodes.push(Node { id });
        id
    }
    pub fn add_edge(&mut self, nodes: Vec<NodeId>) -> usize {
        let id = self.edges.len();
        self.edges.push(HyperEdge { id, nodes });
        id
    }
}
