use crate::taffy::{layout::Cache, prelude::Layout, style::Style};

/// Copy of taffy's `NodeData` for custom implementation of `Taffy`[taffy::Taffy] with `SecondaryMap`[slotmap::secondary::SecondaryMap]s
pub(crate) struct StyleData {
    /// The layout strategy used by this node
    pub(crate) style: Style,
    /// The results of the layout computation
    pub(crate) layout: Layout,
    /// The primary cached results of the layout computation
    pub(crate) size_cache: [Option<Cache>; 4],
}

impl StyleData {
    /// Create the data for a new node
    #[must_use]
    pub const fn new(style: Style) -> Self {
        Self {
            style,
            size_cache: [None; 4],
            layout: Layout::new(),
        }
    }

    /// Marks a node and all of its parents (recursively) as dirty
    ///
    /// This clears any cached data and signals that the data must be recomputed.
    #[inline]
    pub fn mark_dirty(&mut self) {
        self.size_cache = [None; 4];
    }
}
