use bevy::prelude::*;

mod collider;
mod components;
mod configs;
mod entities;

fn main() {
    App::new()
        .add_plugins(configs::ConfigsPlugin)
        .add_plugins(entities::EntitiesPlugin)
        .run();
}
