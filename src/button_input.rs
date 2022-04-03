use std::path::Path;

use bevy::prelude::*;
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

                    *color = Default::default();
                } else {
                    // if its disabled, enable it
                    current_run_json.enable_deck(deck_num.num);

                    *color = ENABLED_DECK.into();
                }
            } else {
                if !enabled_json.check_disabled(&deck_num.num) {
                    // if its enabled, disable it
                    enabled_json.disable(deck_num.num);
                    *color = DISABLE_DECK.into();
                } else {
                    // if its disabled, enable it
                    enabled_json.disable(deck_num.num);

                    *color = Default::default();
                }
            }
        }
    }
}

pub fn handle_ui_buttons(
    mut state: ResMut<State<GameState>>,
    mut curr_page: ResMut<CurrPage>, // current index in deck selection
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
                    MenuItems::Right => {
                        if (curr_page.index + 1) * 8 > num_decks {
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
                   MenuItems::Save => {
                       enabled_json.update(); // store struct in file
                   }
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
