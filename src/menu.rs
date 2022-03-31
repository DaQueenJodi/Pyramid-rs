use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

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
        app.insert_resource(MenuData { button_entity: vec![] });
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_menu));
        app.add_system_set(
            SystemSet::on_update(GameState::MainMenu).with_system(handle_ui_buttons),
        );
        app.add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(close_menu));
    }
}
#[derive(Clone, Copy)]
pub enum MenuItems {
    Play,
    DeckSelection,
    Settings,
    Quit,
}

struct MenuData {
    button_entity: Vec<Entity>,
}

fn handle_ui_buttons(
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                state.set(GameState::InGame).unwrap();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn setup_menu(mut commands: Commands, mut asset_server: ResMut<AssetServer>, mut menu_data: ResMut<MenuData>) {
    let font: Handle<Font> = asset_server.load("fonts/Roboto.ttf");

    menu_data.button_entity.push(commands.spawn_bundle(TextBundle {
        style: Style {
            position: Rect {
                bottom: Val::Px(800.0), // for some reason bevy seems to use rect.bottom for the y value?
                left: Val::Px(715.0),   // see above
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::with_section(
            "Main Menu",
            TextStyle {
                font: font.clone(),
                font_size: 100.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),

        ..Default::default()
    }).id());
    menu_data.button_entity.push(
    spawn_button(
        &mut commands,
        &mut asset_server,
        "Play",
        40.0,
        820.0,
        400.0,
        Vec2::new(200.0, 100.0),
    ));
    menu_data.button_entity.push(
    spawn_button(
        &mut commands,
        &mut asset_server,
        "Settings",
        40.0,
        820.0,
        600.0,
        Vec2::new(200.0, 100.0),
    ));
}

fn spawn_button(
    commands: &mut Commands,
    asset_server: &mut ResMut<AssetServer>,
    text: &str,
    font_size: f32,
    posx: f32,
    posy: f32,
    size: Vec2,
) -> Entity {
   commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(size[0]), Val::Px(size[1])),
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(posy),
                    left: Val::Px(posx),
                    ..Default::default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    text,
                    TextStyle {
                        font: asset_server.load("fonts/Roboto.ttf"),
                        font_size: font_size,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        }).id()
}

fn close_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    for i in &menu_data.button_entity {
        commands.entity(*i).despawn_recursive();
    }
}
