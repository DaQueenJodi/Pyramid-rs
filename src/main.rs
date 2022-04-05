#![allow(clippy::redundant_field_names)]
use bevy::{prelude::*, window::WindowMode};
use bevy_debug_text_overlay::OverlayPlugin;
use button_input::ButtonInputPlugin;
use settings::{Settings, SettingsPlugin};
use std::collections::HashMap;
pub mod deck;
use deck::*;
pub mod debug;
use debug::*;
pub mod states_and_ui;
use states_and_ui::*;
pub mod handle_json;
use handle_json::*;
pub mod button_input;
pub mod constants;
use constants::*;
pub mod actual_game;
pub mod settings;
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
            width: 960.0,
            height: 640.0,
            title: "The Pyramid".to_owned(),
            vsync: true,
            resizable: false,
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .add_state(GameState::MainMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(OverlayPlugin {
            font_size: 32.0,
            ..Default::default()
        })
        .add_plugin(DebugPlugin)
        .add_plugin(DeckPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(JsonPlugin)
        .add_plugin(StaticMut)
        .add_plugin(SettingsPlugin)
        .add_plugin(ButtonInputPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(setup_game)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup_game(
    mut enabled_json: ResMut<EnabledJson>,
    mut deck_data: ResMut<DeckDataWrapper>,
    mut settings: ResMut<Settings>,
) {
    enabled_json.load();
    deck_data.load();
    settings.load();

    for deck in 0..deck_data.decks.len() {
        if !enabled_json.check_disabled(&deck) && !enabled_json.check_enabled(&deck) {
            // if it isnt disabled and isnt already enabled, add it
            enabled_json.enable(deck);
        }
    }
    enabled_json.update();
}
