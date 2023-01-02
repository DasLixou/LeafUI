use std::ops::Deref;

use crate::LeafID;

pub struct LeafRef<D> {
    pub(crate) id: LeafID,
    pub(crate) data: D,
}

impl<D> Into<LeafID> for LeafRef<D> {
    fn into(self) -> LeafID {
        self.id.clone()
    }
}

impl<D> Deref for LeafRef<D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
