std::env::set_current_dir(std::path::Path::new("assets/")).unwrap(); // to make the read_dir command more readable
let paths = fs::read_dir("decks").unwrap();
for path in paths {
    // iterate through every file in assets/decks/
    let file_path = path.unwrap().path(); // turn Path into file path
    let image = assets.load(file_path.clone());
    let (collumns, rows, name) = parse_name(file_path.to_str().unwrap().to_lowercase()); // parse the name (remove the extension off the file)
    println!("Collumns: {}, Rows: {}, Name: {}", collumns, rows, name);
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::new(CARD_H, CARD_W), // the size of the cards
        collumns,
        rows,
        Vec2::new(3.5, 5.0),
    );
    let atlas_handle = texture_atlases.add(atlas);

    let back_path = ("backs/".to_owned() + name.as_str() + ".png") as String;
    let temp_deck = Deck {
        cards: atlas_handle,
        back: assets.load(std::path::Path::new(&back_path)),
        name: name,
        rows: rows,
        collumns: collumns,
    };

    deck_vec.push(temp_deck);
}
commands.insert_resource(Decks(deck_vec));
}


fn parse_name(name: String) -> (usize, usize, String) {
    let mut tuple = (0, 0, String::new());
    let ext_offset = name.rfind(".png").unwrap(); // find first instance of '.png' from the right
    let i_offset = name.rfind("/").unwrap();

    // println!("{}", &name[i_offset+1..i_offset+3]);
    // println!("{}", &name[i_offset+3..i_offset+5]);
    // println!("{}", &name[i_offset+5..ext_offset]);

    tuple.0 = name[i_offset + 1..i_offset + 3].parse().unwrap();
    tuple.1 = name[i_offset + 3..i_offset + 5].parse().unwrap();
    tuple.2 = name[i_offset + 5..ext_offset].parse().unwrap();
    tuple
}