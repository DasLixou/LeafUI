use leafui::{
    nodes::{Label, Stack},
    Leaf, Shrub,
};
use taffy::style::{FlexDirection, Style};

fn main() {
    let mut shrub = Shrub::new();

    let main = content(&mut shrub);

    shrub.run(main);
}

fn content(shrub: &mut Shrub) -> Leaf {
    let text = shrub.new_leaf(Label::new("Hello, World!"), Style::default(), &[]);
    let second_text = shrub.new_leaf(Label::new("And hello again :)"), Style::default(), &[]);

    shrub.new_leaf(
        Stack::new(),
        Style {
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        &[text, second_text],
    )
}

/*#[derive(Debug)]
struct Content;

impl Leaf for Content {
    fn layout(&self, taffy: &mut Taffy) -> Node {
        taffy.new_leaf(Style::default()).unwrap()
    }

    fn design(&self, shrub: &mut Shrub) -> Option<LeafID> {
        Some(
            Padding::new(Rect::from_points(5., 0., 2., 2.))
                .set_child(
                    VStack::new()
                        .add_child(Label::new("Hello, World!").create(shrub))
                        .add_child(Label::new("And hello again :)").create(shrub))
                        .create(shrub),
                )
                .create(shrub),
        )
    }

    fn create(self, shrub: &mut Shrub) -> LeafID {
        shrub.register_leaf(Box::new(self))
    }
}
*/
