#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct LeafID(pub(crate) u64);

impl LeafID {
    pub const UNKNOWN: LeafID = LeafID(0);
}
