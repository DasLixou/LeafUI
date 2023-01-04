use druid_shell::piet::{Color, PaintBrush, Piet, RenderContext, StrokeStyle};
use taffy::{
    prelude::{Layout, Rect},
    style::{AlignItems, FlexDirection, Style},
};

use crate::Blossom;

#[derive(Debug)]
pub struct Stack {
    pub flex_direction: FlexDirection,
    pub align_items: AlignItems,
}

impl Blossom for Stack {
    fn style(&self) -> Style {
        Style {
            flex_direction: self.flex_direction,
            align_items: self.align_items,
            padding: Rect::from_points(2., 2., 2., 2.),
            ..Default::default()
        }
    }

    fn render(&self, layout: &Layout, piet: &mut Piet) {
        println!("Render Stack: {layout:?}");
        piet.stroke_styled(
            druid_shell::kurbo::Rect::new(
                layout.location.x.into(),
                layout.location.y.into(),
                layout.location.x as f64 + layout.size.width as f64,
                layout.location.y as f64 + layout.size.height as f64,
            ),
            &PaintBrush::Color(Color::rgb8(255, 255, 255)),
            1.0,
            &StrokeStyle::new(),
        );
    }
}
