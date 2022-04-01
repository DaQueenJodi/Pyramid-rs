use std::path::Path;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{deck::Decks, NUM_DECKS};

const NORMAL_BUTTON: Color = Color::rgb(0.45, 0.45, 0.45);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const CANT_PRESS_BUTTON: Color = crate::CLEAR;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    PreGame,
    InGame,
    Quit,
    DeckSelection,
    Left,
    Right,
}
#[derive(Component)]
struct MainMenu;
#[derive(Serialize, Deserialize)]
struct EnabledJsonData {
    enabled: Vec<usize>,
}

pub static mut ENABLED_JSON: Vec<usize> = vec![];

pub fn check_json(num: usize) -> bool {
    unsafe { ENABLED_JSON.contains(&num) }
}

pub fn remove_from_json(num: usize) {
    // read from file
    let file_path = Path::new("config/enabled_decks.json");
    let reader = std::fs::File::open(file_path).unwrap();
    let mut json_data: EnabledJsonData =
        serde_json::from_reader(std::io::BufReader::new(reader)).unwrap();

    // taken from https://stackoverflow.com/a/26243276/17942630
    let i = json_data.enabled.iter().position(|x| *x == num).unwrap();
    json_data.enabled.remove(i);

    // write back to file
    let writer = std::fs::File::open(file_path).unwrap();
    serde_json::to_writer(writer, &json_data).unwrap();
}
pub fn add_to_json(num: usize) {
    let file_path = Path::new("config/enabled_decks.json");
    let reader = std::fs::File::open(file_path).unwrap();
    let mut json_data: EnabledJsonData =
        serde_json::from_reader(std::io::BufReader::new(reader)).unwrap();

    json_data.enabled.push(num);

    let writer = std::fs::File::open(file_path).unwrap();
    serde_json::to_writer(writer, &json_data).unwrap();
}

pub fn update_json() {
    let file_path = Path::new("config/enabled_decks.json");
    let json_str = std::fs::read_to_string(file_path).unwrap();
    let json_data: EnabledJsonData = serde_json::from_str(&json_str).unwrap();
    unsafe { ENABLED_JSON = json_data.enabled }
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MenuData {
            // used for keeping track of text/buttons so they can be despawned
            button_entity: vec![],
        })
        .insert_resource(CurrPage {
            // used for indexing the decks in deck selection
            index: 0,
            shown: vec![],
        })
        .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_main_menu))
        .add_system_set(SystemSet::on_enter(GameState::PreGame).with_system(setup_pre_game))
        .add_system_set(SystemSet::on_enter(GameState::DeckSelection).with_system(setup_deck_menu))
        .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(handle_ui_buttons))
        .add_system_set(
            SystemSet::on_update(GameState::DeckSelection)
                .with_system(handle_ui_buttons)
                .with_system(spawn_row_backs),
        )
        .add_system_set(
            SystemSet::on_update(GameState::PreGame)
                .with_system(handle_ui_buttons)
                .with_system(update_pre_game),
        )
        .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(close_menu))
        .add_system_set(SystemSet::on_exit(GameState::DeckSelection).with_system(close_menu));
    }
}
#[derive(Clone, Copy, Component)]
pub enum MenuItems {
    Play,
    DeckSelection,
    Left,
    Right,
    HowToPlay,
    Quit,
}
// stores what button corresponds to what deck in DeckSelection
#[derive(Component)]
struct DeckNumber {
    num: usize,
}

struct MenuData {
    button_entity: Vec<Entity>,
}

struct CurrPage {
    index: usize,
    shown: Vec<Entity>, // stores the current page's deck entities
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
                    MenuItems::Play => state.set(GameState::PreGame).unwrap(),
                    MenuItems::DeckSelection => state.set(GameState::DeckSelection).unwrap(),
                    MenuItems::Quit => state.set(GameState::Quit).unwrap(),
                    MenuItems::Right => {
                        if (curr_page.index + 1) * 8 > unsafe { NUM_DECKS } {
                            *color = CANT_PRESS_BUTTON.into();
                        } else {
                            curr_page.index += 1;
                        }
                    }
                    MenuItems::Left => {
                        if curr_page.index > 0 {
                            curr_page.index -= 1
                        } else {
                            *color = CANT_PRESS_BUTTON.into();
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

    menu_data.button_entity.push(spawn_main_text(
        &mut commands,
        "Main Menu",
        font.clone(),
        5.0,
    ));
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
fn spawn_row_backs(
    mut commands: Commands,
    decks: Res<Decks>,
    page: Res<CurrPage>,
    mut menu_data: ResMut<MenuData>,
) {
    // exit if the resource wasnt just changed
    if !page.is_changed() {
        return;
    }

    let mut x_mul = 1.0;
    let mut y_mul = 1.0;
    let how_many;
    let num_decks = unsafe {crate::NUM_DECKS};
    if num_decks - (page.index * 8) < 8 {
        how_many = num_decks;
    } else {
        how_many = 8;
    }

    for i in page.index..(page.index + how_many) {
        let back = &decks.0.get(i).unwrap().back;
        menu_data
            .button_entity
            .push(spawn_back_grid(&mut commands, back.clone(), i));
    }
}

fn setup_pre_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut page: ResMut<CurrPage>,
    mut menu_data: ResMut<MenuData>,
    decks: Res<Decks>,
) {
    let size = Vec2::new(250.0, 100.0);

    page.index = 0; // reset index as it was used for the deck selection

    let font = asset_server.load("fonts/Roboto.ttf");

    menu_data.button_entity.push(spawn_main_text(
        &mut commands,
        format!("Select {} decks!", unsafe { crate::DECKS_PER_GAME }).as_str(),
        font.clone(),
        -120.0,
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

fn update_pre_game(
    mut commands: Commands,
    mut page: ResMut<CurrPage>,
    decks: Res<Decks>,
) {

    if !page.is_changed() {
        return;
    }
    for i in &page.shown {
        commands.entity(*i).despawn_recursive();
    }
    page.shown.clear();


    let num_decks = unsafe { NUM_DECKS };

    let mut how_many = 0;
    if num_decks - (page.index * 8) < 8 {
        how_many = num_decks - (page.index * 8);
    } else {
        how_many = 8;
    }

    println!("{}", how_many);

    if num_decks < unsafe { crate::DECKS_PER_GAME } {
        unsafe {
            crate::DECKS_PER_GAME = num_decks;
        }
    }
    for i in page.index..(page.index + how_many) {
      //  println!("{}", page.index);
      //  println!("{}", &decks.0.len());
        let back = &decks.0.get(page.index + i).unwrap().back;
        page.shown
            .push(spawn_back_grid(&mut commands, back.clone(), i));
    }
}

fn spawn_back_grid(commands: &mut Commands, image: Handle<Image>, index: usize) -> Entity {
    let mut mulx = 1.0;
    let mut muly = 1.0;
    for i in 0..=index {
        mulx += 1.0;
        if i % 4 == 0 {
            // for every row
            mulx = 1.0;
            muly *= 4.0;
        }
    }

    let image = UiImage::from(image);
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(2100.0 * 0.1), Val::Px(3000.0 * 0.1)),
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(700.0 - (muly * 40.0)),
                    left: Val::Px(300.0 * mulx),
                    ..Default::default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            image: image,
            //color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(DeckNumber { num: index })
        .id()
}
