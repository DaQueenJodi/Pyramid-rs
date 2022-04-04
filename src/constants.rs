use std::path::Path;

use bevy::prelude::*;

use crate::states_and_ui::GameState;

use crate::handle_json::DeckDataWrapper;

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

pub const NUM_COLLUMNS: usize = 5;

pub const SCALE: f32 = 0.7;

pub struct GameGlobals {
    pub decks_per_game: usize,
    pub total_decks: usize,
}

pub struct StaticMut;

impl Plugin for StaticMut {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameGlobals {
            decks_per_game: 5,
            total_decks: 0,
        })
        .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(set_total_decks));
    }
}

fn set_total_decks(mut globals: ResMut<GameGlobals>) {
    let file_path = Path::new("config/decks.json");
    let json_str = std::fs::read_to_string(file_path).unwrap();
    let json_data: DeckDataWrapper = serde_json::from_str(&json_str).unwrap();
    globals.total_decks = json_data.decks.len();
}
