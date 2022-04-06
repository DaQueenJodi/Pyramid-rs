use std::{fs::File, path::Path};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

impl Plugin for JsonPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnabledJson {
            enabled: Vec::new(),
            disabled: Vec::new(),
        })
        .insert_resource(CurrentRunJson {
            score: 0,
            decks: Vec::new(),
            hand: Vec::new(),
        })
        .insert_resource(DeckDataWrapper { decks: Vec::new() });
    }
}

impl DeckDataWrapper {
    pub fn load(&mut self) {
        let file_path = Path::new("config/decks.json");
        let reader = File::open(file_path).unwrap(); // open in read only
        *self = serde_json::from_reader(reader).unwrap();
    }
}

impl EnabledJson {
    pub fn load(&mut self) {
        let file_path = Path::new("config/enabled_decks.json");
        let reader = File::open(file_path).unwrap(); // open in read only
        *self = serde_json::from_reader(reader).unwrap();
    }

    pub fn update(&self) {
        let file_path = Path::new("config/enabled_decks.json");
        let writer = File::options()
            .write(true)
            .truncate(true)
            .open(file_path)
            .unwrap(); // open with write perms
        serde_json::to_writer(writer, self).unwrap();
    }
    pub fn enable(&mut self, deck: usize) {
        self.enabled.push(deck);

        // remove from disabled list if applicable
        if self.check_disabled(&deck) {
            let i = self.disabled.iter().position(|x| *x == deck).unwrap(); // taken from https://stackoverflow.com/a/26243276/17942630
            self.disabled.remove(i);
        }
    }

    pub fn disable(&mut self, deck: usize) {
        self.disabled.push(deck);

        // remove from enabled list if applicable
        if self.check_enabled(&deck) {
            let i = self.enabled.iter().position(|x| *x == deck).unwrap(); // taken from https://stackoverflow.com/a/26243276/17942630
            self.enabled.remove(i);
        }
    }

    pub fn check_enabled(&self, num: &usize) -> bool {
        self.enabled.contains(num)
    }
    pub fn check_disabled(&self, num: &usize) -> bool {
        self.disabled.contains(num)
    }
}
impl CurrentRunJson {
    pub fn load(&mut self) {
        // load json into the struct
        let file_path = Path::new("config/current_run.json");
        let reader = File::open(file_path).unwrap(); // open in read only mode
        *self = serde_json::from_reader(reader).unwrap();
    }

    pub fn update(&self) {
        // update JSON file
        let file_path = Path::new("config/current_run.json");
        let writer = File::options()
            .write(true)
            .truncate(true)
            .open(file_path)
            .unwrap(); // open with write perms
        serde_json::to_writer(writer, self).unwrap(); // write file
    }
    pub fn check_deck(&self, deck: &usize) -> bool {
        // check if deck is in the vector
        self.decks.contains(deck)
    }
    pub fn check_hand(&self, card: &usize) -> bool {
        // check if card is in the vector
        self.hand.contains(card)
    }

    pub fn disable_deck(&mut self, deck: usize) {
        if self.check_deck(&deck) {
            let i = self.decks.iter().position(|x| *x == deck).unwrap(); // taken from https://stackoverflow.com/a/26243276/17942630
            self.decks.remove(i);
        } else {
            println!("Could not find value {} in json", deck);
        }
    }
    pub fn enable_deck(&mut self, deck: usize) {
        self.decks.push(deck);
    }
}

pub struct JsonPlugin;

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentRunJson {
    pub decks: Vec<usize>,
    pub score: usize,
    pub hand: Vec<usize>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct EnabledJson {
    pub disabled: Vec<usize>,
    pub enabled: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeckDataWrapper {
    pub decks: Vec<DeckData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeckData {
    pub primary_cards: usize, // card count

    pub name: String,      // name used in the deck selection
    pub file: String,      // path to sprite sheet
    pub back_file: String, // path to the back image

    pub secondary_cards: usize,
}
