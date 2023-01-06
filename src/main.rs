use leafui::{
    default,
    leaves::{Label, Stack},
    Leaf, Shrub,
};
use taffy::style::{AlignItems, FlexDirection};

fn main() {
    let mut shrub = Shrub::new();

    let main = content(&mut shrub);

    shrub.run(main);
}

fn content(shrub: &mut Shrub) -> Leaf {
    let text = shrub.new_leaf(
        Label {
            text: "Hello, World!".into(),
        },
        &[],
    );
    let second_text = shrub.new_leaf(
        Label {
            text: "And hello again :)".into(),
        },
        &[],
    );

    shrub.new_leaf(
        Stack {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        &[text, second_text],
    )
}
