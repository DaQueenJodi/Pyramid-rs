use crate::constants::*;
use crate::deck::spawn_card;
use crate::{deck::Decks, handle_json::CurrentRunJson, states_and_ui::spawn_back_grid};
use bevy::prelude::*;

pub fn setup_actual_game(
    mut commands: Commands,
    current_run_json: Res<CurrentRunJson>,
    mut decks: ResMut<Decks>,
) {
    let startx = -600.0;
    let starty = 200.0;

    let mut mulx = 1.0;
    let mut muly = 1.0;

    let card_x = 200.0;
    let card_y = 300.0;

    for deck in 0..current_run_json.decks.len() {
        let index_img = 0; // TODO make this a random number

        if deck % 4 == 0 {
            mulx = 1.0;
            muly -= 1.0;
        }

        if deck % 2 == 1 {
            // every other iteration
            mulx += 2.0;
        }

        let x = startx + (mulx * card_x);
        let y = starty + (muly * card_y);

        if deck % 2 == 1 {
            // every other iteration
            mulx -= 2.0;
        }

        spawn_card(
            &mut commands,
            &mut decks,
            deck,
            index_img,
            Vec3::new(x, y, 0.0),
            true,
        );

        spawn_card(
            &mut commands,
            &mut decks,
            deck,
            index_img,
            Vec3::new(x - card_x, y, 0.0),
            false,
        );

        mulx += 1.0;
    }
}
