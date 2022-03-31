use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::MainMenu)
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu).with_system(handle_ui_buttons),
            )
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_menu));
    }
}


pub enum MenuItems {
    Play,
    Deck_Selection,
    Settings,
}


fn handle_ui_buttons(commands: Commands, buttons: Res<Input<MouseButton>>, windows: Res<Windows>) {
    
}

fn setup_menu(commands: Commands) {
    
}

fn close_menu(commands: Commands) {}
