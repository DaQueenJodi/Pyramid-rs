use std::{fs::File, path::Path};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};


impl EnabledJson {

    pub fn update(&mut self) {
        let file_path = Path::new("config/enabled_decks.json");
        let reader = File::open(file_path).unwrap();
        let temp_json: Self = serde_json::from_reader(std::io::BufReader::new(reader)).unwrap();
        self.enabled = temp_json.enabled;
    }
    pub fn check(&self, num: &usize) -> bool {
        self.enabled.contains(num)
    }
}
impl  CurrentRunJson {
   pub fn update(&mut self) {
        let file_path = Path::new("config/current_run.json");
        let reader = File::open(file_path).unwrap();
        let temp_json: Self = serde_json::from_reader(std::io::BufReader::new(reader)).unwrap();
        *self = temp_json; // update original json
    }
    pub fn check_deck(&self, deck: &usize) -> bool { // check if deck is in the vector
        self.decks.contains(deck)
    }
    pub fn check_hand(&self, card: &usize) -> bool { // check if card is in the vector
        self.hand.contains(card)
    }
}

pub struct JsonPlugin;

impl Plugin for JsonPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnabledJson {
            enabled: Vec::new(),
        })
        .insert_resource(CurrentRunJson {
            score: 0,
            decks: Vec::new(),
            hand: Vec::new(),
        });
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentRunJson {
    pub decks: Vec<usize>,
    pub score: usize,
    pub hand: Vec<usize>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct EnabledJson {
    pub enabled: Vec<usize>,
}

pub fn disable_deck(deck: usize) {
    // read from file
    let file_path = Path::new("config/enabled_decks.json");
    let reader = File::open(file_path).unwrap();
    let mut json_data: EnabledJson =
        serde_json::from_reader(std::io::BufReader::new(reader)).unwrap();

    // taken from https://stackoverflow.com/a/26243276/17942630
    let i = json_data.enabled.iter().position(|x| *x == deck).unwrap();
    json_data.enabled.remove(i);

    // write back to file
    let writer = File::create(file_path).unwrap();
    serde_json::to_writer_pretty(writer, &json_data).unwrap();
}
pub fn enable_deck(deck: usize) {
    let file_path = Path::new("config/enabled_decks.json");
    let reader = File::open(file_path).unwrap(); // open file in read only mode
    let mut json_data: EnabledJson =
        serde_json::from_reader(std::io::BufReader::new(reader)).unwrap();

    json_data.enabled.push(deck);

    let writer = File::create(file_path).unwrap(); // open file in writable mode
    serde_json::to_writer_pretty(writer, &json_data).unwrap();
}
