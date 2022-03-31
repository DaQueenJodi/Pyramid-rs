use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use crate::{CARD_H, CARD_W, SCALE};

#[derive(Default, Component, Inspectable, Clone, Debug)]
pub struct Deck {
    pub cards: Handle<TextureAtlas>,
    pub name: String,
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
    mut texture_ids: ResMut<crate::SpriteSheetIds>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut deck_vec: Vec<Deck> = vec![];

    //std::env::set_current_dir(std::path::Path::new("assets/")).unwrap(); // to make the read_dir command more readable
    // iterate through every file in assets/decks/ that ends in .json_data
    let file_path = Path::new("config/decks.json");
    let json_str = fs::read_to_string(file_path).unwrap();
    let json_data: DeckDataWrapper = serde_json::from_str(&json_str).unwrap();
    for curr_json in json_data.decks {
        let cards = curr_json.cards;
        let texture_ids2 = &texture_ids.ids;
        if !texture_ids2.contains_key(&curr_json.file) {
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
            let temp_deck = Deck {
                cards: atlas_handle.clone(),
                name: curr_json.name,
                num_cards: cards,
                offset: curr_json.offset,
            };
            deck_vec.push(temp_deck);
            println!("inserted: {:?}", curr_json.file);

            texture_ids.ids.insert(curr_json.file, atlas_handle); // insert file path so it doesnt get duplicated for no reason
        } else {
            let temp_deck = Deck {
                cards: texture_ids.ids.get(&curr_json.file).unwrap().clone(),
                name: curr_json.name,
                num_cards: cards,
                offset: curr_json.offset,
            };
            deck_vec.push(temp_deck);
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
    let mut sprite = TextureAtlasSprite::new(index + deck.offset);
    // println!("{}", index + (deck.offset));
    // println!("{:#?}", deck);
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
