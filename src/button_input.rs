use std::path::Path;

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use bevy_debug_text_overlay::screen_print;

use crate::{constants::*, handle_json::*, states_and_ui::*};

pub fn handle_choosing_cards(
    state: ResMut<State<GameState>>,
    mut enabled_json: ResMut<EnabledJson>,
    mut current_run_json: ResMut<CurrentRunJson>,
    mut interaction_query: Query<
        (&Interaction, &DeckNumber, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    if enabled_json.is_changed() {
        return;
    }

    for (interaction, deck_num, mut color) in interaction_query.iter_mut() {
        if interaction == &Interaction::Clicked {
            //screen_print!("Deck Selected: {}", deck_num.num);

            if *state.current() == GameState::PreGame {
                if current_run_json.check_deck(&deck_num.num) {
                    // if its enabled, disable it
                    current_run_json.disable_deck(deck_num.num);

                    screen_print!("disabled deck: {}", deck_num.num);

                    *color = Default::default();
                } else {
                    // if its disabled, enable it
                    current_run_json.enable_deck(deck_num.num);
                    *color = ENABLED_DECK.into();

                    screen_print!("enabled deck: {}", deck_num.num);
                }
            } else {
                if enabled_json.check_enabled(&deck_num.num) {
                    // if its enabled, disable it
                    enabled_json.disable(deck_num.num);
                    *color = DISABLE_DECK.into();

                    screen_print!("disabled deck: {}", deck_num.num);
                } else {
                    // if its disabled, enable it
                    enabled_json.disable(deck_num.num);

                    *color = Default::default();
                    screen_print!("enabled deck: {}", deck_num.num);
                }
            }
        }
    }
}

pub fn handle_ui_buttons(
    mut state: ResMut<State<GameState>>,
    globals: Res<GameGlobals>,
    enabled_json: Res<EnabledJson>,
    mut current_run_json: ResMut<CurrentRunJson>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &MenuItems),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, menu_items) in interaction_query.iter_mut() {
        let mut num_decks = 0;

        match state.current() {
            GameState::DeckSelection => num_decks = globals.total_decks, // total decks
            GameState::PreGame => num_decks = globals.total_decks - enabled_json.disabled.len(), // enabled decks
            _ => {}
        }

        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();

                match menu_items {
                    MenuItems::HowToPlay => state.set(GameState::HowTo).unwrap(),
                    MenuItems::Continue => {
                        if !Path::new("config/current_run.json").exists() {
                            // check if save data exists
                            *color = CANT_PRESS_BUTTON.into();
                        } else {
                            current_run_json.load();
                            state.set(GameState::PreGame).unwrap();
                        }
                    }
                    MenuItems::NewGame => {
                        // delete the save file if it exists, then enter pre-game
                        let path = Path::new("config/current_run.json");
                        if path.exists() {
                            std::fs::remove_file(path).unwrap();
                        }
                        state.set(GameState::PreGame).unwrap();
                    }
                    MenuItems::DeckSelection => state.set(GameState::DeckSelection).unwrap(),
                    MenuItems::Quit => state.set(GameState::Quit).unwrap(),
                    MenuItems::Save => {
                        enabled_json.update(); // store struct in file
                    }
                    _ => {}
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

pub fn spawn_button(
    commands: &mut Commands,
    font: Handle<Font>,
    text: &str,
    font_size: f32,
    posx: f32,
    posy: f32,
    size: Vec2,
    button_type: MenuItems,
    color: Color,
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
            color: color.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    text,
                    TextStyle {
                        font,
                        font_size,
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

pub fn spawn_button_img(
    commands: &mut Commands,
    size: Vec2,
    posx: f32,
    posy: f32,
    button_type: MenuItems,
    img: Handle<Image>,
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
            image: img.into(),
            ..Default::default()
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

#[macro_export]
macro_rules! spawn_button_grid {
    (
        $commands:expr, $font:expr,
        $(($but_type:expr, $text:expr)),+
    ) => { {
            let mut entities: Vec<Entity> = Vec::new();

            let mut y = 100.0;

            $( entities.push(spawn_button($commands, $font, $text,40.0, 820.0, y, Vec2::new(250.0, 100.0), $but_type, NORMAL_BUTTON)) ; y += 200.0;)+
            entities
    }};
}

pub fn scroll_deckmap(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<&mut Style, With<Scrollable>>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut style) in query.iter_mut() {
            style.position.bottom += mouse_wheel_event.y * 40.0; // move up/down depending on how much the mouse moved
        }
    }
}
