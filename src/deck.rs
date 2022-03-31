use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use crate::{CARD_H, CARD_W, SCALE};

#[derive(Default, Component, Inspectable, Clone, Debug)]
pub struct Deck {
    pub sheet: Handle<TextureAtlas>,
    pub cards: usize,
    pub offset: usize,
}
#[derive(Clone)]
pub struct DecksTogether {
    pub primary: Deck,
    pub secondary: Deck,
    pub name: String,
    pub back: Handle<Image>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DeckDataWrapper {
    decks: Vec<DeckData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DeckData {
    primary_cards: usize,
    primary_offset: usize,

    name: String,
    file: String,

    secondary_cards: usize,
    secondary_offset: usize,
}

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, make_decks);
    }
}

pub struct Decks(pub Vec<DecksTogether>);

pub fn make_decks(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_ids: ResMut<crate::SpriteSheetIds>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut deck_vec: Vec<DecksTogether> = vec![];
    let file_path = Path::new("config/decks.json");
    let json_str = fs::read_to_string(file_path).unwrap();
    let json_data: DeckDataWrapper = serde_json::from_str(&json_str).unwrap();
    for curr_json in json_data.decks.clone() {
        if !texture_ids.ids.contains_key(&curr_json.file.clone()) {
            // if file hasnt already made a handle

            let image: Handle<Image> = assets.load(Path::new(&curr_json.file));

            let atlas = TextureAtlas::from_grid_with_padding(
                image,
                Vec2::new(CARD_H, CARD_W), // the size of the cards
                10,
                5,
                Vec2::new(3.5, 5.0),
            );
            let atlas_handle = texture_atlases.add(atlas);

            deck_vec.push(gen_2_decks(curr_json.clone(), atlas_handle.clone()));

            texture_ids.ids.insert(curr_json.file, atlas_handle); // insert file path so it doesnt get duplicated for no reason
        } else {
            let texture = texture_ids.ids.get(&curr_json.file).unwrap().clone();

            deck_vec.push(gen_2_decks(curr_json, texture));
        }
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
    let mut sprite = TextureAtlasSprite::new(index + deck.offset);
    sprite.custom_size = Some(Vec2::new(CARD_H * SCALE, CARD_W * SCALE));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: deck.sheet.clone(),
            transform: Transform {
                translation: translation,
                scale: Vec3::new(SCALE, SCALE, SCALE), // scale the height and width
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}

fn gen_2_decks(json: DeckData, texture: Handle<TextureAtlas>) -> DecksTogether {
    let name = json.name;
    let primary_cards = json.primary_cards;
    let secondary_cards = json.secondary_cards;
    let primary_offset = json.primary_offset;
    let secondary_offset = json.secondary_offset;

    let temp_deck1 = Deck {
        sheet: texture.clone(),
        cards: primary_cards,
        offset: primary_offset,
    };

    let temp_deck2 = Deck {
        sheet: texture,
        cards: secondary_cards,
        offset: secondary_offset,
    };

    DecksTogether {
        primary: temp_deck1,
        secondary: temp_deck2,
        name: name,
    }
}
