use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
}
#[derive(Component)]
struct MainMenu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::MainMenu)
            .add_startup_system_to_stage(StartupStage::PreStartup, setup_menu);
        //     .add_system_set(
        //         SystemSet::on_update(GameState::MainMenu).with_system(handle_ui_buttons),
        //     )
        //     .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_menu));
    }
}
#[derive(Clone, Copy)]
pub enum MenuItems {
    Play,
    DeckSelection,
    Settings,
    Quit,
}

fn handle_ui_buttons(commands: Commands, buttons: Res<Input<MouseButton>>, windows: Res<Windows>) {}

fn setup_menu(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let font = asset_server.load("fonts/Roboto.ttf");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                display: Display::None,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Default::default()
            },
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert(MainMenu)
        .with_children(|mut parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Main Menu",
                    TextStyle {
                        font: font.clone(),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..Default::default() // TextBundle
            });
        });
}

fn close_menu(commands: Commands) {}
