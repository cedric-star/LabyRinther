use bevy::prelude::*;
use crate::util::ui::constants::*;

#[derive(Component, Clone)]
pub struct ButtonImages {
    pub normal: Handle<Image>,
    pub hovered: Handle<Image>,
    pub pressed: Handle<Image>,
}

impl ButtonImages {
    pub fn load(asset_server: &AssetServer) -> Self {
        Self {
            normal: asset_server.load("btn_normal.png"),
            hovered: asset_server.load("btn_hovered.png"),
            pressed: asset_server.load("btn_pressed.png"),
        }
    }
}

pub fn button_visuals(
    mut query: Query<
        (&Interaction, &ButtonImages, &mut ImageNode),
        Changed<Interaction>,
    >,
) {
    for (interaction, images, mut image_node) in &mut query {
        image_node.image = match *interaction {
            Interaction::Pressed => images.pressed.clone(),
            Interaction::Hovered => images.hovered.clone(),
            _ => images.normal.clone(),
        };
    }
}

pub fn my_button(
    marker: impl Bundle,
    label: &str,
    height: f32,
    images: &ButtonImages,
) -> impl Bundle {
    (
        Button,
        marker,
        Node {
            width: px(300),
            height: px(height),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(px(8)),
            ..default()
        },
        ImageNode::new(images.normal.clone()),
        images.clone(),
        children![(
            Text::new(label.to_string()),
            TextFont { font_size: FontSize::Px(33.0), ..default() },
            TextColor(TXT_COL),
        )],
    )
}
