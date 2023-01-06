use slotmap::{SecondaryMap, SlotMap, SparseSecondaryMap};
use taffy::{
    error::TaffyError,
    layout::Cache,
    node::MeasureFunc,
    prelude::{AvailableSpace, Layout, Size},
    style::Style,
    tree::LayoutTree,
};
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::{style_data::StyleData, Blossom};

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
        Self::with_capacity(16)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            leaves: SlotMap::with_capacity(capacity),
            children: SecondaryMap::with_capacity(capacity),
            parents: SecondaryMap::with_capacity(capacity),
            styles: SecondaryMap::with_capacity(capacity),
            measure_funcs: SparseSecondaryMap::with_capacity(capacity),
        }
    }

    pub fn new_leaf(&mut self, blossom: impl Blossom, children: &[Leaf]) -> Leaf {
        let style = blossom.style();

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

        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title("LeafUI Example")
            .build(&event_loop)
            .unwrap();

        event_loop.run(move |event, _, control_flow| {
            control_flow.set_wait();

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    WindowEvent::MouseInput {
                        state: ElementState::Released,
                        ..
                    } => {
                        window.request_redraw();
                    }
                    WindowEvent::Resized(new_inner_size) => {
                        println!("calculate size (resized)");
                        self.compute_layout(leaf, {
                            Size {
                                width: AvailableSpace::Definite(new_inner_size.width as f32),
                                height: AvailableSpace::Definite(new_inner_size.height as f32),
                            }
                        })
                        .unwrap();
                    }
                    WindowEvent::ScaleFactorChanged {
                        scale_factor: _,
                        new_inner_size,
                    } => {
                        println!("calculate size (factor changed)");
                        self.compute_layout(leaf, {
                            Size {
                                width: AvailableSpace::Definite(new_inner_size.width as f32),
                                height: AvailableSpace::Definite(new_inner_size.height as f32),
                            }
                        })
                        .unwrap();
                    }
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    println!("\nredrawing!\n");
                    self.render(leaf);
                }
                _ => (),
            }
        });
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
