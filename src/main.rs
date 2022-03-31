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

pub const SCALE: f32 = 0.7;

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
        .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(spawn_row))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn spawn_row(mut commands: Commands, decks: Res<Decks>) {
    let mut x = -2.5 * CARD_W;
    let mut y = 2.5 * CARD_H;
    let deck_num = 4;
    let primary = false;
    let deck;

    if primary {
        deck = &decks.0.get(deck_num).unwrap().primary;
    } else {
        deck = &decks.0.get(deck_num).unwrap().secondary;
    }

    for i in 0..(deck.cards) {
        if i % NUM_COLLUMNS == 0 {
            y -= CARD_H + 100.0;
            x = -2.5 * (CARD_W);
        }
        x += CARD_W;
        //println!("index: {}, x: {}, y: {}", i, x, y);
        deck::spawn_card(
            &mut commands,
            &decks,
            deck_num,
            i,
            Vec3::new(x * SCALE, y * SCALE, 0.0),
            true,
        );
    }
}
