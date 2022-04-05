use bevy::prelude::*;
use configparser::ini::Ini;

use crate::{button_input::{spawn_main_text, spawn_button_img, spawn_button}, spawn_button_grid, states_and_ui::{MenuItems, MenuData}};


#[derive(Clone, Copy)]
pub enum SettingsItems {
    Layout,
    UI,
}

struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Settings {
            settings: Ini::new(),
        });
    }



}

impl Settings {
    pub fn load(&mut self) {
        self.settings.load("config/settings.ini").unwrap();
    }

    pub fn update(&self) {
        self.settings.write("config/settings.ini").unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub settings: Ini, // store Ini config
}


pub fn setup_settings(mut commands: Commands, asset_server: Res<AssetServer>, settings: Res<Settings>, mut menu_data: ResMut<MenuData>) {

    let font: Handle<Font> = asset_server.load("fonts/Roboto.ttf");

    menu_data.button_entity.push(spawn_main_text(
        &mut commands,
        "Settings",
        font.clone(),
        5.0,
    ));
    // spawn buttons and add them to the button entity vector to despawn later

    menu_data.button_entity.push(spawn_button_img(
        &mut commands,
        Vec2::new(100.0, 100.0),
        100.0,
        900.0,
        MenuItems::Settings,
        asset_server.load("ui/settings_cog.png"),
    ))
}


