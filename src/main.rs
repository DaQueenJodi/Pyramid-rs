#![allow(clippy::redundant_field_names)]
use bevy::prelude::*;

pub mod deck;
use deck::*;
pub mod debug;
use debug::*;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;

pub const CARD_H: f32 = 406.0;
pub const CARD_W: f32 = 580.0;

pub const NUM_ROWS: usize = 4;
pub const NUM_COLLUMNS: usize = 6; 

pub const SCALE: f32 = 0.7;



fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: 1920.0,
            height: 1280.0,
            title: "*notices your bulgy wulgy* uwu".to_owned(),
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(spawn_row)
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(DeckPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_row(mut commands: Commands, mut decks: Res<Decks>) {
    let mut x =  -4.0 * CARD_W;
    let mut y = 2.5 * CARD_H;

    for i in 0..40 {
        if i % NUM_COLLUMNS == 0 {
            y -= CARD_H + 100.0;
            x =  -4.0 * (CARD_W);
        }
        x += CARD_W;
        println!("index: {}, x: {}, y: {}", i, x, y);
        deck::spawn_card(&mut commands, &mut decks, 1, i, Vec3::new(x * SCALE, y * SCALE, 0.0));
    }
}
