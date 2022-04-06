use bevy::{input::mouse::MouseWheel, prelude::*};
use std::path::Path;

use crate::{constants::*, handle_json::*, settings::{LayoutSettings, Colors}, states_and_ui::*};

pub struct LastMenu {
    pub last: GameState,
}
pub struct ButtonInputPlugin;

impl Plugin for ButtonInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LastMenu {
            last: GameState::MainMenu,
        });
    }
}

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

                    *color = Default::default();
                } else {
                    // if its disabled, enable it
                    current_run_json.enable_deck(deck_num.num);
                    *color = ENABLED_DECK.into();
                }
            } else {
                if enabled_json.check_enabled(&deck_num.num) {
                    // if its enabled, disable it
                    enabled_json.disable(deck_num.num);
                    *color = DISABLED_DECK.into();
                } else {
                    // if its disabled, enable it
                    enabled_json.enable(deck_num.num);

                    *color = Default::default();
                }
            }
        }
    }
}

pub fn handle_ui_buttons(
    mut state: ResMut<State<GameState>>,
    mut last_menu: ResMut<LastMenu>,
    enabled_json: Res<EnabledJson>,
    colors: Res<Colors>,
    mut current_run_json: ResMut<CurrentRunJson>,
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
                    MenuItems::HowToPlay => state.set(GameState::HowTo).unwrap(),
                    MenuItems::Continue => {
                        if !Path::new("config/current_run.json").exists() {
                            // check if save data exists
                            *color = colors.disabled_button.into();
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
                    MenuItems::DeckSelection => {
                        last_menu.last = GameState::MainMenu;
                        state.set(GameState::DeckSelection).unwrap()
                    }
                    MenuItems::Quit => state.set(GameState::Quit).unwrap(),
                    MenuItems::Save => enabled_json.update(), // store struct in file
                    MenuItems::Play => state.set(GameState::InGame).unwrap(),
                    MenuItems::Back => {
                        match *state.current() {
                            GameState::InGame => {
                                // TODO make dialogue that asks you to save or quit
                                current_run_json.update();
                                state.set(GameState::MainMenu).unwrap();
                            }
                            _ => {
                                if *state.current() != last_menu.last {
                                    // make sure you dont go to the same state, causes runtime error
                                    state.set(last_menu.last).unwrap()
                                }
                            }
                        }
                    }
                    MenuItems::Settings => state.set(GameState::Settings).unwrap(),
                    _ => {}
                }
            }

            Interaction::Hovered => {
                *color = colors.hovered_button.into();
            }
            Interaction::None => {
                *color = colors.normal_button.into();
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

            let mut _y = 700.0;

            $( entities.push(spawn_button($commands, $font, $text,40.0, 820.0, _y, Vec2::new(250.0, 100.0), $but_type)) ; _y -= 200.0;)+
            entities
    }};
}

// scroll the pre-game and deck selection menus
pub fn scroll_backmap(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<&mut Style, With<Scrollable>>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for mut style in query.iter_mut() {
            style.position.bottom += mouse_wheel_event.y * -70.0; // move up/down depending on how much the mouse moved (in reverse because it feels better)
        }
    }
}

// scrolls the map during the actual game
pub fn scroll_gamemap(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    layout: Res<LayoutSettings>,
    mut query: Query<&mut Transform, With<Scrollable>>,
) {
    let mul = 70.0;

    for mouse_wheel_event in mouse_wheel_events.iter() {
        for mut transform in query.iter_mut() {
            if !layout.vertical {
                transform.translation.y += mouse_wheel_event.y * -mul; // move up/down depending on how much the mouse moved (in reverse because it feels better)
            } else {
                transform.translation.x += mouse_wheel_event.y * mul;
            }
        }
    }
}
