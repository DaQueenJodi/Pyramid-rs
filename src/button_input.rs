use bevy::prelude::*;

use crate::{constants::*, handle_json::*, states_and_ui::*};

pub fn handle_choosing_cards(
    mut enabled_json: ResMut<EnabledJson>,
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
            if !enabled_json.check_disabled(&deck_num.num) {
                // if its enabled, disable it
                disable_deck(deck_num.num);
                enabled_json.update();

                *color = DISABLE_DECK.into();
            } else {
                // if its disabled, enable it
                enable_deck(deck_num.num);
                enabled_json.update();

                *color = Default::default();
            }
        }
    }
}

pub fn handle_ui_buttons(
    mut state: ResMut<State<GameState>>,
    mut curr_page: ResMut<CurrPage>, // current index in deck selection
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &MenuItems),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, menu_items) in interaction_query.iter_mut() {
        let mut num_decks = 0;

        match state.current() {
            GameState::DeckSelection => num_decks = unsafe { NUM_DECKS }, // total decks
            GameState::PreGame => num_decks = unsafe { DECKS_PER_GAME },  // enabled decks
            _ => {}
        }

        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();

                match menu_items {
                    MenuItems::HowToPlay => state.set(GameState::HowTo).unwrap(),
                    MenuItems::Play => state.set(GameState::PreGame).unwrap(),
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
