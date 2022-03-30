use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use crate::{CARD_H, CARD_W, SCALE};

#[derive(Default, Component, Inspectable, Clone)]
pub struct Deck {
    pub cards: Handle<TextureAtlas>,
    pub name: String,
    pub rows: usize,
    pub collumns: usize,
    pub num_cards: usize,
    pub offset: usize,
}
#[derive(Serialize, Deserialize, Debug)]
struct DeckDataWrapper {
    decks: Vec<DeckData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DeckData {
    name: String,
    collumns: usize,
    cards: usize,
    file: String,
    offset: usize,
}

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, make_decks);
    }
}

pub struct Cards(pub Handle<Image>);

pub struct Decks(pub Vec<Deck>);

pub fn make_decks(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut deck_vec: Vec<Deck> = vec![];

    //std::env::set_current_dir(std::path::Path::new("assets/")).unwrap(); // to make the read_dir command more readable
    let paths = fs::read_dir("assets/decks").unwrap();
    for path in paths {
        // iterate through every file in assets/decks/ that ends in .json_data
        let file_path = path.unwrap().path(); // turn Path into file path
        if file_path.to_str().unwrap().ends_with(".json") {
            println!("file name: {:#?}", file_path);

            let json_str = fs::read_to_string(&file_path).unwrap();
            let json_data: DeckDataWrapper = serde_json::from_str(&json_str).unwrap();
            for curr_json in json_data.decks {
                let cards = curr_json.cards;
                let collumns = curr_json.collumns;
                let rows = cards / collumns;

                let image: Handle<Image> = assets.load(Path::new(&curr_json.file));

                let atlas = TextureAtlas::from_grid_with_padding(
                    image,
                    Vec2::new(CARD_H, CARD_W), // the size of the cards
                    collumns,
                    rows,
                    Vec2::new(3.5, 5.0),
                );
                let atlas_handle = texture_atlases.add(atlas);
                let temp_deck = Deck {
                    cards: atlas_handle,
                    name: curr_json.name,
                    rows: rows,
                    num_cards: cards,
                    collumns: collumns,
                    offset: curr_json.offset,
                };
                deck_vec.push(temp_deck);
            }
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
) -> Entity {
    let deck = decks.0.get(deck_num).unwrap();
    let mut sprite = TextureAtlasSprite::new(index + (deck.offset));
    sprite.custom_size = Some(Vec2::new(CARD_H * SCALE, CARD_W * SCALE));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: deck.cards.clone(),
            transform: Transform {
                translation: translation,
                scale: Vec3::new(SCALE, SCALE, SCALE), // scale the height and width
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}
