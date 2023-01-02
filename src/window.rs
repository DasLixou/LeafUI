use std::any::Any;

use druid_shell::{
    kurbo::Size,
    piet::{Color, Piet, RenderContext},
    Region, WinHandler, WindowHandle,
};

#[derive(Default)]
pub struct Window {
    size: Size,
    handle: WindowHandle,
}

const BG_COLOR: Color = Color::rgb8(0x27, 0x28, 0x22);

impl WinHandler for Window {
    fn connect(&mut self, handle: &WindowHandle) {
        self.handle = handle.clone();
    }

    fn prepare_paint(&mut self) {}

    fn paint(&mut self, piet: &mut Piet, _: &Region) {
        piet.fill(self.size.to_rect(), &BG_COLOR);
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
