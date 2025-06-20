use bevy::prelude::*;

mod movement;
mod setup;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup::setup)
            .add_systems(Update, movement::movement);
    }
}
