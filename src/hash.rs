use std::hash::{Hash, Hasher};
use kuchiki::{NodeDataRef, ElementData, NodeRef, Node};

pub struct HashableNodeRef {
    pub node: NodeRef,
}

impl HashableNodeRef {
    pub fn new(node: &NodeDataRef<ElementData>) -> HashableNodeRef {
        HashableNodeRef { node: node.as_node().clone() }
    }
}

impl Hash for HashableNodeRef {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        let a: *const Node = &*self.node.0;
        a.hash(state)
    }
}

impl Eq for HashableNodeRef {}
impl PartialEq<HashableNodeRef> for HashableNodeRef {
    fn eq(&self, other: &HashableNodeRef) -> bool {
        let a: *const Node = &*self.node.0;
        let b: *const Node = &*other.node.0;
        a == b
    }
}
