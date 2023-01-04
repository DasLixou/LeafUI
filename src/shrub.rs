use druid_shell::{Application, WindowBuilder};
use slotmap::{SecondaryMap, SlotMap, SparseSecondaryMap};
use taffy::{
    error::TaffyError,
    layout::Cache,
    node::MeasureFunc,
    prelude::{AvailableSpace, Layout, Size},
    style::Style,
    tree::LayoutTree,
};

use crate::{style_data::StyleData, window::Window, Blossom};

pub type Leaf = slotmap::DefaultKey;

struct LeafData {
    blossom: Box<dyn Blossom>,
}

pub struct Shrub {
    leaves: SlotMap<Leaf, LeafData>,
    children: SecondaryMap<Leaf, Vec<Leaf>>,
    parents: SecondaryMap<Leaf, Option<Leaf>>,
    styles: SecondaryMap<Leaf, StyleData>,
    /// The mapping from the Size<AvailableSpace> (in real units) to Size<f32> (in points) for this node
    measure_funcs: SparseSecondaryMap<Leaf, MeasureFunc>,
}

impl Shrub {
    pub fn new() -> Self {
        Self {
            leaves: SlotMap::with_capacity(1),
            children: SecondaryMap::with_capacity(1),
            parents: SecondaryMap::with_capacity(1),
            styles: SecondaryMap::with_capacity(1),
            measure_funcs: SparseSecondaryMap::with_capacity(1),
        }
    }

    pub fn new_leaf(&mut self, blossom: impl Blossom, style: Style, children: &[Leaf]) -> Leaf {
        let leaf = self.leaves.insert(LeafData {
            blossom: Box::new(blossom),
        });
        self.styles.insert(leaf, StyleData::new(style));
        self.parents.insert(leaf, None);

        self.children.insert(leaf, Vec::from(children));
        for child in children {
            self.parents[*child] = Some(leaf);
        }

        leaf
    }

    pub fn run(mut self, leaf: Leaf) {
        self.compute_layout(leaf, {
            Size {
                width: AvailableSpace::Definite(1280.),
                height: AvailableSpace::Definite(720.),
            }
        })
        .unwrap();

        self.render(leaf);

        let app = Application::new().unwrap();
        let mut builder = WindowBuilder::new(app.clone());
        builder.set_handler(Box::<Window>::default());
        builder.set_title("Hello example");

        let window = builder.build().unwrap();
        window.show();

        app.run(None);
    }

    pub fn render(&self, leaf: Leaf) {
        let layout = self.layout(leaf);
        self.leaves[leaf].blossom.render(layout);

        let children = self.children[leaf].as_slice();
        for leaf in children {
            self.render(*leaf);
        }
    }

    fn compute_layout(
        &mut self,
        node: Leaf,
        available_space: Size<AvailableSpace>,
    ) -> Result<(), TaffyError> {
        taffy::compute_layout(self, node, available_space)
    }
}

impl LayoutTree for Shrub {
    fn children(&self, node: Leaf) -> &[Leaf] {
        self.children[node].as_slice()
    }

    fn child(&self, node: Leaf, index: usize) -> Leaf {
        self.children[node][index]
    }

    fn parent(&self, node: Leaf) -> Option<Leaf> {
        self.parents[node]
    }

    fn style(&self, node: Leaf) -> &Style {
        &self.styles[node].style
    }

    fn layout(&self, node: Leaf) -> &Layout {
        &self.styles[node].layout
    }

    fn layout_mut(&mut self, node: Leaf) -> &mut Layout {
        &mut self.styles[node].layout
    }

    fn mark_dirty(&mut self, node: taffy::prelude::Node) -> taffy::error::TaffyResult<()> {
        /// WARNING: this will stack-overflow if the tree contains a cycle
        fn mark_dirty_recursive(
            styles: &mut SecondaryMap<Leaf, StyleData>,
            parents: &SecondaryMap<Leaf, Option<Leaf>>,
            leaf: Leaf,
        ) {
            styles[leaf].mark_dirty();

            if let Some(Some(node)) = parents.get(leaf) {
                mark_dirty_recursive(styles, parents, *node);
            }
        }

        mark_dirty_recursive(&mut self.styles, &self.parents, node);

        Ok(())
    }

    fn measure_node(
        &self,
        node: Leaf,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
    ) -> Size<f32> {
        match &self.measure_funcs[node] {
            MeasureFunc::Raw(measure) => measure(known_dimensions, available_space),

            MeasureFunc::Boxed(measure) => {
                (measure as &dyn Fn(_, _) -> _)(known_dimensions, available_space)
            }
        }
    }

    fn needs_measure(&self, node: Leaf) -> bool {
        self.styles[node].needs_measure && self.measure_funcs.get(node).is_some()
    }

    fn cache_mut(&mut self, node: Leaf, index: usize) -> &mut Option<Cache> {
        &mut self.styles[node].size_cache[index]
    }
}
