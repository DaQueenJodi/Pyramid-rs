use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use std::{fs, path::Path};

use crate::{
    handle_json::{CurrentRunJson, DeckData, DeckDataWrapper},
    states_and_ui::{DeckNumber, Scrollable},
    CARD_H, CARD_W, SCALE,
};

#[derive(Default, Component, Inspectable, Clone, Debug)]
pub struct Deck {
    pub sheet: Handle<TextureAtlas>,
    pub cards: usize,
    pub offset: usize,
}
#[derive(Clone, Debug)]
pub struct DecksTogether {
    pub primary: Deck,
    pub secondary: Deck,
    pub name: String,
}

#[derive(Default, Clone)]
pub struct DeckBacks {
    pub backs: Vec<Handle<Image>>,
}

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, make_backs)
            .insert_resource(DeckBacks { backs: Vec::new() });
    }
}

#[derive(Debug)]
pub struct Decks(pub Vec<DecksTogether>);

pub fn make_decks(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    current_run_json: Res<CurrentRunJson>,
    deck_data: Res<DeckDataWrapper>,
) {
    let mut deck_vec: Vec<DecksTogether> = Vec::new();

    let mut index = 0;

    for curr_json in deck_data.decks.clone() {
        // if it is not enabled in the current run, dont add it
        if !current_run_json.check_deck(&index) {
            index += 1;
            continue;
        }

        // if file hasnt already made a handle

        let image: Handle<Image> = assets.load(&curr_json.file);

        let atlas = TextureAtlas::from_grid_with_padding(
            image,
            Vec2::new(CARD_H, CARD_W), // the size of the cards
            10,
            5,
            Vec2::new(3.5, 5.0),
        );
        let atlas_handle = texture_atlases.add(atlas);

        deck_vec.push(gen_2_decks(curr_json.clone(), atlas_handle.clone()));

        index += 1;
    }
    commands.insert_resource(Decks(deck_vec));
}
pub fn spawn_card(
    commands: &mut Commands,
    decks: &Decks,
    deck_num: usize,
    index: usize,
    translation: Vec3,
    primary: bool,
) -> Entity {
    let deck;
    if primary {
        deck = &decks.0.get(deck_num).unwrap().primary;
    } else {
        deck = &decks.0.get(deck_num).unwrap().secondary;
    }
    let sprite = TextureAtlasSprite::new(index + deck.offset);
    //sprite.custom_size = Some(Vec2::new(CARD_H * SCALE, CARD_W * SCALE));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: deck.sheet.clone(),
            transform: Transform {
                translation,
                scale: Vec3::new(SCALE, SCALE, SCALE), // scale the height and width
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(DeckNumber { num: index })
        .insert(Scrollable {}) // TODO make it not forced to be scrollable, this is just convenient
        .id()
}

fn gen_2_decks(json: DeckData, texture: Handle<TextureAtlas>) -> DecksTogether {
    let name = json.name;
    let primary_cards = json.primary_cards;
    let secondary_cards = json.secondary_cards;
    let secondary_offset = json.primary_cards; // the secondaries will always be right after the primaries

    let temp_deck1 = Deck {
        sheet: texture.clone(),
        cards: primary_cards,
        offset: 0, // primary offset is always 0
    };

    let temp_deck2 = Deck {
        sheet: texture,
        cards: secondary_cards,
        offset: secondary_offset,
    };

    DecksTogether {
        primary: temp_deck1,
        secondary: temp_deck2,
        name,
    }
}

pub fn make_backs(assets: Res<AssetServer>, mut deck_backs: ResMut<DeckBacks>) {
    let file_path = Path::new("config/decks.json");
    let json_str = fs::read_to_string(file_path).unwrap();
    let json_data: DeckDataWrapper = serde_json::from_str(&json_str).unwrap();
    for curr_json in json_data.decks.clone() {
        let back: Handle<Image> = assets.load(Path::new(&curr_json.back_file));

        deck_backs.backs.push(back);
    }
}
