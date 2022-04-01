use crate::{button_input::*, constants::*, handle_json::*};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

use crate::DECKS_PER_GAME;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MenuData {
            // used for keeping track of text/buttons so they can be despawned
            button_entity: Vec::new(),
        })
        .insert_resource(CurrPage {
            // used for indexing the decks in deck selection
            index: 0,
            shown: Vec::new(),
        })
        .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_main_menu))
        .add_system_set(SystemSet::on_enter(GameState::PreGame).with_system(setup_pre_game))
        .add_system_set(
            SystemSet::on_update(GameState::DeckSelection).with_system(handle_choosing_cards),
        )
        .add_system_set(SystemSet::on_enter(GameState::DeckSelection).with_system(setup_deck_menu))
        .add_system_set(
            SystemSet::on_update(GameState::DeckSelection).with_system(handle_choosing_cards),
        )
        .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(handle_ui_buttons))
        .add_system_set(
            SystemSet::on_update(GameState::DeckSelection)
                .with_system(handle_ui_buttons)
                .with_system(update_deck_select),
        )
        .add_system_set(
            SystemSet::on_update(GameState::PreGame)
                .with_system(handle_ui_buttons)
                .with_system(handle_choosing_cards)
                .with_system(update_pre_game),
        )
        .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(close_menu))
        .add_system_set(SystemSet::on_exit(GameState::DeckSelection).with_system(close_menu))
        .add_system_set(SystemSet::on_exit(GameState::PreGame).with_system(crate::make_decks));
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    HowTo,
    PreGame,
    InGame,
    Quit,
    DeckSelection,
    Left,
    Right,
}
#[derive(Component)]
pub struct MainMenu;
#[derive(Serialize, Deserialize)]

pub struct MenuPlugin;

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
#[derive(Component, Inspectable, Default, Debug)]
pub struct DeckNumber {
    pub num: usize,
}

struct MenuData {
    button_entity: Vec<Entity>,
}

pub struct CurrPage {
    pub index: usize,
    pub shown: Vec<Entity>, // stores the current page's deck entities
}

fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
    asset_server: Res<AssetServer>,
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

pub fn display_how_to() {}

pub fn spawn_button(
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

pub fn spawn_main_text(
    commands: &mut Commands,
    text: &str,
    font: Handle<Font>,
    offset: f32,
) -> Entity {
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
fn update_deck_select(
    mut commands: Commands,
    decks: Res<crate::deck::DeckBacks>,
    mut page: ResMut<CurrPage>,
    enabled_json: Res<EnabledJson>,
) {
    let index = page.index;

    // exit if the resource wasnt just changed
    if !page.is_changed() {
        return;
    }

    for i in &page.shown {
        commands.entity(*i).despawn_recursive();
    }
    page.shown.clear();

    let num_decks = unsafe { crate::NUM_DECKS };
    let how_many;
    if num_decks - (page.index * 8) < 8 {
        how_many = num_decks - (page.index * 8);
    } else {
        how_many = 8;
    }

    for i in index..(index + how_many) {
        // set deck color to normal, otherwise make it disabled
        let mut color = UiColor::default();
        if enabled_json.check_disabled(&(i - index)) {
            color = DISABLE_DECK.into();
        }
        let deck_num = (index * 7) + i;
        let back = decks.backs.get(deck_num).unwrap();
        page.shown.push(spawn_back_grid(
            &mut commands,
            back.clone(),
            i - index,
            deck_num,
            color,
        ));
    }
}

fn setup_pre_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut page: ResMut<CurrPage>,
    mut menu_data: ResMut<MenuData>,
    mut enabled_json: ResMut<EnabledJson>,
) {
    enabled_json.update(); // make sure that the whitelist is set properly

    unsafe { DECKS_PER_GAME = enabled_json.enabled.len() };

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
    deck_backs: Res<crate::DeckBacks>,
) {
    if !page.is_changed() {
        return;
    }
    for i in &page.shown {
        commands.entity(*i).despawn_recursive();
    }
    page.shown.clear();

    let num_decks = unsafe { DECKS_PER_GAME };

    let how_many;
    if num_decks - (page.index * 8) < 8 {
        how_many = num_decks - (page.index * 8);
    } else {
        how_many = 8;
    }

    //println!("{}", how_many);

    if num_decks < unsafe { crate::DECKS_PER_GAME } {
        unsafe {
            crate::DECKS_PER_GAME = num_decks;
        }
    }

    let index = page.index;

    for i in index..(index + how_many) {
        let deck_num = (index * 7) + i;
        let back = deck_backs.backs.get(deck_num).unwrap();

        page.shown.push(spawn_back_grid(
            &mut commands,
            back.clone(),
            i - index,
            deck_num,
            UiColor::default(),
        ));
    }
}

pub fn spawn_back_grid(
    commands: &mut Commands,
    image: Handle<Image>,
    index: usize,
    deck_num: usize,
    color: UiColor,
) -> Entity {
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
                    left: Val::Px(100.0 + (300.0 * mulx)),
                    ..Default::default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            image: image,
            color: color,
            ..Default::default()
        })
        .insert(DeckNumber { num: deck_num })
        .id()
}
