
use crate::deck::spawn_card;
use crate::settings::LayoutSettings;
use crate::{deck::Decks, handle_json::CurrentRunJson};
use bevy::prelude::*;

pub fn setup_actual_game(
    mut commands: Commands,
    current_run_json: Res<CurrentRunJson>,
    mut decks: ResMut<Decks>,
    layout: Res<LayoutSettings>,
) {
    let vertical = layout.vertical;

    // if vertical is true; increase starty, decrease mulx_multiplier, and make number of collumns 4 instead of 4

    let startx = match vertical {
        // controls the starting x position
        true => -600.0,
        false => -600.0,
    };
    let starty = match vertical {
        // controls the starting y position
        true => 900.0,
        false => 900.0,
    };

    let mut mulx = 0.0;
    let mut muly = -1.0;

    let mulx_adder = match vertical {
        // controls how much is added to mulx every iteration
        true => 1.0,
        false => 2.0,
    };

    let muly_minuser = match vertical {
        true => 2.0,
        false => 1.0,
    };

    let card_x = 400.0;
    let card_y = 600.0;

    let collumns = 2;

    for deck in 0..current_run_json.decks.len() {
        let index_img = 0; // TODO make this a random number

        if !vertical && deck != 0 && deck % collumns == 0 {
            mulx = 0.0;
            muly -= muly_minuser;
        }

        let mut x = startx + (mulx * card_x);
        let mut y = starty + (muly * card_y);

        spawn_card(
            &mut commands,
            &mut decks,
            deck,
            index_img,
            Vec3::new(x, y, 0.0),
            true,
        );

        if !vertical {
            x += card_x;
        } else {
            y -= card_y;
        }

        spawn_card(
            &mut commands,
            &mut decks,
            deck,
            index_img,
            Vec3::new(x, y, 0.0),
            false,
        );
        mulx += mulx_adder;
    }
}
