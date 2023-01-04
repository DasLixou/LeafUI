use std::any::Any;

use druid_shell::{
    kurbo::Size,
    piet::{Color, Piet, RenderContext},
    Region, WinHandler, WindowHandle,
};

use crate::{Leaf, Shrub};

#[derive(Default)]
pub struct Window {
    pub(crate) size: Size,
    pub(crate) handle: WindowHandle,
    pub(crate) shrub: Option<Shrub>,
    pub(crate) leaf: Option<Leaf>,
}

const BG_COLOR: Color = Color::rgb8(0x27, 0x28, 0x22);

impl WinHandler for Window {
    fn connect(&mut self, handle: &WindowHandle) {
        self.handle = handle.clone();
    }

    fn prepare_paint(&mut self) {}

    fn paint(&mut self, piet: &mut Piet, _: &Region) {
        piet.fill(self.size.to_rect(), &BG_COLOR);
        self.shrub.as_ref().map(|s| {
            s.render(self.leaf.unwrap(), piet);
        });
    }

    fn size(&mut self, size: Size) {
        self.size = size;
    }

    fn request_close(&mut self) {
        self.handle.close();
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
