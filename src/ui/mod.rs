use bevy::{
    app::{App, Update},
    color::Color,
};

use crate::ui::button::update_button_display;

pub mod button;
pub mod label;

pub const BLACK: Color = Color::srgb(0.0, 0.0, 0.0);

pub fn ui_plugin(app: &mut App) {
    app.add_systems(Update, update_button_display);
}
