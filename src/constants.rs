use bevy::prelude::*;

pub const NORMAL_BUTTON: Color = Color::rgb(0.45, 0.45, 0.45);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35); // green
pub const CANT_PRESS_BUTTON: Color = crate::CLEAR;

pub const DISABLE_DECK: Color = Color::rgb(0.75, 0.35, 0.35); // red
pub const ENABLED_DECK: Color = Color::rgb(0.35, 0.75, 0.35); // green

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;

pub const CARD_H: f32 = 406.0;
pub const CARD_W: f32 = 580.0;

pub const NUM_ROWS: usize = 4;
pub const NUM_COLLUMNS: usize = 4;

pub static mut NUM_DECKS: usize = 0;

pub const SCALE: f32 = 0.7;

pub static mut DECKS_PER_GAME: usize = 5;
