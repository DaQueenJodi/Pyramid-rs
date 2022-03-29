use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Default, Component, Inspectable)]
pub struct Wheel {
    pub choices: Vec<Choice>
}

#[derive(Default, Inspectable)]
pub struct Choice {

}

pub struct WheelPlugin;

impl Plugin for WheelPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_wheel)
        .add_system(rotate_wheel);
        
    }
}

fn rotate_wheel(mut query: Query<(&Wheel, &mut Transform,)>) {
    let (wheel, mut transform) = query.single_mut();
    let mut angle = transform.rotation.to_axis_angle();
    angle.1 += 10.0;
    transform.rotation = Quat::from_rotation_z(angle.1);
    
}

fn spawn_wheel(mut commands: Commands) {
    let wheel = create_wheel(&mut commands);
    commands.entity(wheel)
        .insert(Name::new("Wheel"))
        .insert(Wheel{ choices: vec![] })
        .id();

}

fn create_wheel(mut commands: &Commands) -> Entity {
    
}