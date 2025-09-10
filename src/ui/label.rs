use bevy::{ecs::bundle::Bundle, text::TextColor, ui::widget::Text};

use crate::ui::BLACK;

pub fn label(text: &str) -> impl Bundle + use<> {
    (Text(text.into()), TextColor(BLACK))
}
