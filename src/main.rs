#![allow(clippy::redundant_field_names) ]
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::math::{Quat, Vec2};
use bevy::{ecs::schedule::SystemSet};


pub mod wheel;
use wheel::*;
pub mod debug;
use debug::*;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0/9.0;
fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: 720.0,
            height: 480.0,
            title: "Wheel of Isaac".to_owned(),
            vsync: true,
            resizable: false,
            ..Default::default()


        })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(WheelPlugin)
        .add_startup_system(spawn_camera)
        .run();
}
#[derive(Default)]
struct Game {
    wheel: Wheel,
    items: Vec<i32>
}



fn rotate_wheel(mut commands: Commands, mut game: Res<Game>) {

}

fn spawn_camera(mut commands: Commands) {
    // let mut camera = OrthographicCameraBundle::new_2d();

    // camera.orthographic_projection.top = 1.0;
    // camera.orthographic_projection.bottom = -1.0;

    // camera.orthographic_projection.right = 1.0 * RESOLUTION;
    // camera.orthographic_projection.left = -1.0 * RESOLUTION;

    // camera.orthographic_projection.scaling_mode = ScalingMode::None;
    // commands.spawn_bundle(camera);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}