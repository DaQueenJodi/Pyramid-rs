use bevy::{
    ecs::{archetype::Archetypes, component::Components},
    prelude::*,
    reflect::TypeRegistration,
};

// shamelessly stolen from (https://bevy-cheatbook.github.io/cookbook/print-resources.html)
pub fn set_resources(archetypes: &Archetypes, components: &Components) {
    let mut r: Vec<String> = archetypes
        .resource()
        .components()
        .map(|id| components.get_info(id).unwrap())
        // get_short_name removes the path information
        // i.e. `bevy_audio::audio::Audio` -> `Audio`
        // if you want to see the path info replace
        // `TypeRegistration::get_short_name` with `String::from`
        .map(|info| TypeRegistration::get_short_name(info.name()))
        .collect();

    // sort list alphebetically
    r.sort();
    //  unsafe { crate::RESOURCES = r };
    //r.iter().for_each(|name| println!("{}", name));
}
