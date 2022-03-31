use bevy::prelude::*;

use crate::{CARD_W, CARD_H, NUM_COLLUMNS, deck::{self, Decks}, SCALE};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
    Quit,
    Settings,
    DeckSelection,
    Left,
    Right,
}
#[derive(Component)]
struct MainMenu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MenuData {
            // used for keeping track of text/buttons so they can be despawned
            button_entity: vec![],
        });
        app.insert_resource(CurrPage {
            // used for indexing the decks in DeckSelection
            index: 0,
        });
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_main_menu));
        app.add_system_set(
            SystemSet::on_enter(GameState::Settings).with_system(setup_settings_menu),
        );
        app.add_system_set(
            SystemSet::on_enter(GameState::DeckSelection).with_system(setup_deck_menu),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::MainMenu).with_system(handle_ui_buttons),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Settings).with_system(handle_ui_buttons),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::DeckSelection).with_system(handle_ui_buttons),
        );
        app.add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(close_menu));
        app.add_system_set(SystemSet::on_exit(GameState::Settings).with_system(close_menu));
        app.add_system_set(SystemSet::on_exit(GameState::DeckSelection).with_system(close_menu));
    }
}
#[derive(Clone, Copy, Component)]
pub enum MenuItems {
    Play,
    DeckSelection,
    Left,
    Right,
    Settings,
    HowToPlay,
    Quit,
}

struct MenuData {
    button_entity: Vec<Entity>,
}

struct CurrPage {
    index: usize,
}

fn handle_ui_buttons(
    mut state: ResMut<State<GameState>>,
    mut curr_page: ResMut<CurrPage>, // current index in deck selection
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &MenuItems),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, menu_items) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();

                match menu_items {
                    MenuItems::HowToPlay => display_how_to(),
                    MenuItems::Settings => state.set(GameState::Settings).unwrap(),
                    MenuItems::Play => state.set(GameState::InGame).unwrap(),
                    MenuItems::DeckSelection => state.set(GameState::DeckSelection).unwrap(),
                    MenuItems::Quit => state.set(GameState::Quit).unwrap(),
                    MenuItems::Right => curr_page.index += 1,
                    MenuItems::Left => {
                        if curr_page.index > 0 {
                            curr_page.index -= 1
                        }
                    } // make sure you dont underflow the index
                }
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

fn setup_main_menu(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut menu_data: ResMut<MenuData>,
) {
    let size = Vec2::new(250.0, 100.0);
    let font: Handle<Font> = asset_server.load("fonts/Roboto.ttf");

    menu_data
        .button_entity
        .push(spawn_main_text(&mut commands, "Main Menu", font.clone(), 5.0));
    menu_data.button_entity.push(spawn_button(
        &mut commands,
        font.clone(),
        "Play",
        40.0,
        820.0,
        600.0,
        size,
        MenuItems::Play,
    ));
    menu_data.button_entity.push(spawn_button(
        &mut commands,
        font.clone(),
        "Decks",
        40.0,
        820.0,
        400.0,
        size,
        MenuItems::DeckSelection,
    ));
    menu_data.button_entity.push(spawn_button(
        &mut commands,
        font.clone(),
        "How To Play",
        40.0,
        820.0,
        200.0,
        size,
        MenuItems::HowToPlay,
    ));
}

fn setup_deck_menu(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut menu_data: ResMut<MenuData>,
) {
    let font: Handle<Font> = asset_server.load("fonts/Roboto.ttf");
    let size = Vec2::new(250.0, 100.0);
    menu_data.button_entity.push(spawn_main_text(
        &mut commands,
        "Deck Selection",
        font.clone(),
        -70.0,
    ));
    menu_data.button_entity.push(spawn_button(
        &mut commands,
        font.clone(),
        "Left",
        40.0,
        0.0,
        500.0,
        size,
        MenuItems::Left,
    ));
    menu_data.button_entity.push(spawn_button(
        &mut commands,
        font.clone(),
        "Right",
        40.0,
        1670.0, // somehow this is the rightmost part of the screen idk
        500.0,
        size,
        MenuItems::Right,
    ));
}
fn setup_settings_menu(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut menu_data: ResMut<MenuData>,
) {
}

fn display_how_to() {}

fn spawn_button(
    commands: &mut Commands,
    font: Handle<Font>,
    text: &str,
    font_size: f32,
    posx: f32,
    posy: f32,
    size: Vec2,
    button_type: MenuItems,
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
                        font: font,
                        font_size: font_size,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(button_type)
        .id()
}

fn spawn_main_text(commands: &mut Commands, text: &str, font: Handle<Font>, offset: f32) -> Entity {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position: Rect {
                    bottom: Val::Px(900.0), // for some reason bevy seems to use rect.bottom for the y value?
                    left: Val::Px(715.0 + offset), // see above
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                text,
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
        })
        .id()
}

// delete every button/text we created
fn close_menu(mut commands: Commands, mut menu_data: ResMut<MenuData>) {
    for i in &menu_data.button_entity {
        commands.entity(*i).despawn_recursive();
    }
    menu_data.button_entity.clear();
}
// spawns every card's back as a button
fn spawn_row_backs(mut commands: Commands, decks: Res<Decks>, page: Res<CurrPage>) {
    
    let image = 

    commands.spawn_bundle(ButtonBundle {
        image: 
    }
}
