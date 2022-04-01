#![allow(clippy::redundant_field_names)]
use bevy::{asset::HandleId, prelude::*};
use std::collections::HashMap;
pub mod deck;
use deck::*;
pub mod debug;
use debug::*;
pub mod menu;
use menu::*;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;

pub const CARD_H: f32 = 406.0;
pub const CARD_W: f32 = 580.0;

pub const NUM_ROWS: usize = 4;
pub const NUM_COLLUMNS: usize = 4;

pub static mut NUM_DECKS: usize = 0;

pub const SCALE: f32 = 0.7;

pub static mut DECKS_PER_GAME: usize = 5;

pub struct SpriteSheetIds {
    pub ids: HashMap<String, Handle<TextureAtlas>>,
}

fn main() {
    App::new()
        .insert_resource(SpriteSheetIds {
            ids: HashMap::new(),
        })
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: 1920.0,
            height: 1280.0,
            title: "*notices your bulgy wulgy* uwu".to_owned(),
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_state(GameState::MainMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(DeckPlugin)
        .add_plugin(MenuPlugin)
        .add_startup_system(spawn_camera)
        // .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_game))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
