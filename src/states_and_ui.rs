use crate::{
    button_input::*,
    constants::*,
    deck::{make_decks, DeckBacks},
    handle_json::*,
};
use bevy::prelude::*;
use bevy_debug_text_overlay::screen_print;
use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

use crate::spawn_button_grid;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MenuData {
            // used for keeping track of text/buttons so they can be despawned
            button_entity: Vec::new(),
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
                .with_system(scroll_deckmap)
                .with_system(handle_ui_buttons),
        )
        .add_system_set(
            SystemSet::on_update(GameState::PreGame)
                .with_system(scroll_deckmap)
                .with_system(handle_ui_buttons)
                .with_system(handle_choosing_cards),
        )
        .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(close_menu))
        .add_system_set(SystemSet::on_exit(GameState::DeckSelection).with_system(close_menu))
        .add_system_set(
            SystemSet::on_exit(GameState::PreGame)
                .with_system(close_menu)
                .with_system(make_decks),
        );
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    HowTo,
    PreGame,
    InGame,
    Quit,
    DeckSelection,
    Left,
    Right,

    Loading,
}
#[derive(Component)]
pub struct MainMenu;
#[derive(Serialize, Deserialize)]

pub struct MenuPlugin;

#[derive(Clone, Copy, Component)]
pub enum MenuItems {
    Back,
    Continue,
    NewGame,
    DeckSelection,
    Save,
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

fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut menu_data: ResMut<MenuData>,
    mut enabled_json: ResMut<EnabledJson>,
) {
    spawn_button_img(
        &mut commands,
        Vec2::new(150.0, 150.0),
        100.0,
        800.0,
        MenuItems::Back,
        asset_server.load("ui/back_arrow.png"),
    );

    enabled_json.load(); // load saved enabled decks

    let font: Handle<Font> = asset_server.load("fonts/Roboto.ttf");

    menu_data.button_entity.push(spawn_main_text(
        &mut commands,
        "Main Menu",
        font.clone(),
        5.0,
    ));
    // spawn buttons and add them to the button entity vector to despawn later
    for i in spawn_button_grid!(
        &mut commands,
        font.clone(),
        (MenuItems::HowToPlay, "How To Play"),
        (MenuItems::DeckSelection, "Deck Select"),
        (MenuItems::NewGame, "New Game"),
        (MenuItems::Continue, "Continue")
    ) {
        menu_data.button_entity.push(i);
    }
}

fn setup_deck_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut menu_data: ResMut<MenuData>,
    globals: ResMut<GameGlobals>,
    enabled_json: Res<EnabledJson>,
    deck_backs: Res<DeckBacks>,
) {
    let font: Handle<Font> = asset_server.load("fonts/Roboto.ttf");
    let size = Vec2::new(250.0, 100.0);
    let text = spawn_main_text(&mut commands, "Deck Selection", font.clone(), -70.0);

    commands.entity(text).insert(Scrollable {}); // make this scroll with the cards. looks super weird otherwise
    menu_data.button_entity.push(text);

    menu_data.button_entity.push(spawn_button(
        &mut commands,
        font.clone(),
        "Save",
        40.0,
        0.0, // right between the cards
        0.0,
        size,
        MenuItems::Save,
        NORMAL_BUTTON,
    ));

    // make deck grid
    for i in 0..globals.total_decks {
        // set deck color to normal, otherwise make it disabled
        let mut color = UiColor::default();

        if enabled_json.check_disabled(&i) {
            color = DISABLE_DECK.into();
        } else {
        }
        let back = deck_backs.backs.get(i).unwrap();
        menu_data
            .button_entity
            .push(spawn_back_grid(&mut commands, back.clone(), i, i, color));
    }
}

pub fn display_how_to() {}

// delete every button/text we created
fn close_menu(mut commands: Commands, mut menu_data: ResMut<MenuData>) {
    for i in &menu_data.button_entity {
        commands.entity(*i).despawn_recursive();
    }
    menu_data.button_entity.clear();
}
// spawns every card's back as a button

fn setup_pre_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut menu_data: ResMut<MenuData>,
    enabled_json: Res<EnabledJson>,
    deck_backs: Res<DeckBacks>,
    mut current_run_json: ResMut<CurrentRunJson>,
) {
    let font = asset_server.load("fonts/Roboto.ttf");

    let text = spawn_main_text(
        &mut commands,
        format!("Select {} decks!", enabled_json.enabled.len()).as_str(),
        font.clone(),
        -120.0,
    );

    menu_data
        .button_entity
        .push(commands.entity(text).insert(Scrollable {}).id()); // make this scroll with the cards. looks super weird otherwise

    let mut i = 0;
    for j in enabled_json.enabled.iter() {
        current_run_json.enable_deck(*j);
        let back = deck_backs.backs.get(*j).unwrap();

        menu_data.button_entity.push(spawn_back_grid(
            &mut commands,
            back.clone(),
            i,
            *j,
            UiColor::default(),
        ));
        i += 1;
    }
}

pub fn spawn_back_grid(
    commands: &mut Commands,
    image: Handle<Image>,
    index: usize,
    deck_num: usize,
    color: UiColor,
) -> Entity {
    // start position
    let start_y = 1350.0;
    let start_x = -30.0;

    // incrementers for next locations
    let card_y = 400.0;
    let card_x = 300.0;

    let mut mulx = 1.0;
    let mut muly = 1.0;

    // calculate multipliers based on how many iterations have passed
    for i in 0..=index {
        mulx += 1.0;
        if i % NUM_COLLUMNS == 0 {
            // for every row
            mulx = 1.0;
            muly += 1.0;
        }
    }

    let image = UiImage::from(image);
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(210.0), Val::Px(300.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(start_y - (card_y * muly)),
                    left: Val::Px(start_x + (card_x * mulx)),
                    ..Default::default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            image,
            color,
            ..Default::default()
        })
        .insert(DeckNumber { num: deck_num })
        .insert(Scrollable {})
        .id()
}

#[derive(Component)]
pub struct Scrollable {
    // track the cards
}
