#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct NodeId(u8);

impl NodeId {
    pub fn new(node_id: u8) -> Option<Self> {
        if node_id <= 127 {
            Some(NodeId(node_id))
        } else {
            None
        }
    }

    pub unsafe fn new_unchecked(node_id: u8) -> Self {
        Self(node_id)
    }

    pub const fn raw(&self) -> u8 {
        self.0
    }

    pub fn node_id_mask() -> u16 {
        0x7F
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self(126)
    }
}
