use bevy::prelude::*;
use configparser::ini::Ini;

use crate::{
    button_input::{handle_ui_buttons, spawn_button, spawn_button_img, spawn_main_text, LastMenu},
    constants::*,
    spawn_button_grid,
    states_and_ui::{close_menu, GameState, MenuData, MenuItems},
};

pub enum SettingsSubmenus {
    None,
    UI,
    Layout,
}

#[derive(Clone, Copy)]
pub enum SettingsItems {
    SubItem,
    Layout,
    UI,
}

pub struct SettingsPage {
    page: SettingsSubmenus,
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Settings {
            settings: Ini::new(),
        })
        .insert_resource(SettingsPage {
            page: SettingsSubmenus::None,
        })
        .add_system_set(SystemSet::on_enter(GameState::Settings).with_system(setup_settings))
        .add_system_set(
            SystemSet::on_update(GameState::Settings)
                .with_system(handle_ui_buttons)
                .with_system(handle_settings_input),
        )
        .add_system_set(SystemSet::on_exit(GameState::Settings).with_system(close_menu))
        .add_system_set(SystemSet::on_enter(GameState::SettingsSubmenu).with_system(setup_submenu))
        .add_system_set(
            SystemSet::on_update(GameState::SettingsSubmenu)
                .with_system(handle_ui_buttons)
                .with_system(handle_settings_input),
        )
        .add_system_set(SystemSet::on_exit(GameState::SettingsSubmenu).with_system(close_menu));
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

pub fn setup_settings(
    mut last_menu: ResMut<LastMenu>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<Settings>,
    mut menu_data: ResMut<MenuData>,
) {

    last_menu.last = GameState::MainMenu;

    let font: Handle<Font> = asset_server.load("fonts/Roboto.ttf");

    menu_data.button_entity.push(spawn_main_text(
        &mut commands,
        "Settings",
        font.clone(),
        50.0,
    ));

    for entity in spawn_button_grid!(
        &mut commands,
        font.clone(),
        (MenuItems::SettingsItems(SettingsItems::UI), "UI"),
        (MenuItems::SettingsItems(SettingsItems::Layout), "layout")
    ) {
        menu_data.button_entity.push(entity);
    }
}

fn handle_settings_input(
    mut last_menu: ResMut<LastMenu>,
    mut settings_page: ResMut<SettingsPage>,
    mut state: ResMut<State<GameState>>,
    query: Query<(&Interaction, &MenuItems), (With<Button>, Changed<Interaction>)>,
) {
    for (interaction, menu_item) in query.iter() {
        if *interaction == Interaction::Clicked {
            match menu_item {
                MenuItems::SettingsItems(SettingsItems::UI) => {
                    last_menu.last = GameState::Settings;
                    settings_page.page = SettingsSubmenus::UI;
                    state.set(GameState::SettingsSubmenu).unwrap();
                }
                _ => {}
            }
        }
    }
}

fn setup_submenu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    submenu: Res<SettingsPage>,
    mut menu_data: ResMut<MenuData>,
    mut last_menu: ResMut<LastMenu>
) {

    match submenu.page {
        SettingsSubmenus::UI => {
            let font = asset_server.load("fonts/Roboto.ttf");
            for entity in spawn_button_grid!(
                &mut commands,
                font,
                (MenuItems::SettingsItems(SettingsItems::SubItem), "Color")
            ) {
                menu_data.button_entity.push(entity);
            }
            last_menu.last = GameState::Settings; // TODO add a way to backtrack in the submenu, as easy as adding a special case into handle_button_inputs but im lazy
        }
        _ => {}
    }
}
