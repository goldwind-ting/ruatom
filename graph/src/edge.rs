use std::hash::Hash;
use std::hash::Hasher;

#[derive(Clone, Debug)]
pub struct Edge {
    inbound: u8,
    outbound: u8,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.inbound == other.inbound && self.outbound == other.outbound
    }
}

impl Eq for Edge {}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inbound.hash(state);
        self.outbound.hash(state);
    }
}

impl Edge {
    pub fn new(outbound: u8, inbound: u8) -> Edge {
        Edge { inbound, outbound }
    }

    pub fn inbound(&self) -> &u8 {
        &self.inbound
    }

    /// Returns the inbound u8
    pub fn outbound(&self) -> &u8 {
        &self.outbound
    }
}
