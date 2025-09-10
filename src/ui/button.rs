use bevy::{
    color::Color,
    ecs::{
        bundle::Bundle,
        component::Component,
        hierarchy::Children,
        query::{Changed, With},
        system::Query,
    },
    prelude::*,
    text::TextColor,
    ui::{BackgroundColor, Node, UiRect, Val, widget::Button},
    utils::default,
};

use crate::ui::{BLACK, label::label};

pub const NORMAL_BUTTON: Color = Color::srgb(0.8, 0.8, 0.8);
pub const DISABLED_BUTTON: Color = Color::srgb(0.85, 0.85, 0.85);

pub const DISABLED_TEXT: Color = Color::srgb(0.5, 0.5, 0.5);

#[derive(Component)]
pub struct ButtonState {
    pub enabled: bool,
}

pub fn button(text: &str) -> impl Bundle + use<> {
    (
        Button,
        Node {
            padding: UiRect::all(Val::Px(3.0)),
            ..default()
        },
        BackgroundColor(NORMAL_BUTTON),
        ButtonState { enabled: true },
        children![label(text)],
    )
}

pub fn update_button_display(
    mut buttons: Query<
        (Entity, &mut BackgroundColor, &ButtonState),
        (With<Button>, Changed<ButtonState>),
    >,
    mut labels: Query<&mut TextColor, With<Text>>,
    children: Query<&Children>,
) {
    for (entity, mut background, state) in buttons.iter_mut() {
        for label_id in children.iter_descendants(entity) {
            let Ok(mut text_color) = labels.get_mut(label_id) else {
                continue;
            };
            if state.enabled {
                text_color.0 = BLACK;
            } else {
                text_color.0 = DISABLED_TEXT;
            }
        }
        if state.enabled {
            background.0 = NORMAL_BUTTON;
        } else {
            background.0 = DISABLED_BUTTON;
        }
    }
}
