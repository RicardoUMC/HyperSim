pub type NodeId = usize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    pub id: NodeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperEdge {
    pub id: usize,
    pub nodes: Vec<NodeId>,
}

#[derive(Debug, Default)]
pub struct HyperGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<HyperEdge>,
}

impl HyperGraph {
    pub fn new() -> Self {
        HyperGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    pub fn add_node(&mut self) -> NodeId {
        let node_id = self.nodes.len();
        self.nodes.push(Node { id: node_id });
        node_id
    }
    pub fn add_edge(&mut self, node_ids: Vec<NodeId>) -> usize {
        let edge_id = self.edges.len();
        self.edges.push(HyperEdge {
            id: edge_id,
            nodes: node_ids,
        });
        edge_id
    }
}
