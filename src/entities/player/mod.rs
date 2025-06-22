use bevy::prelude::*;

mod attack;
mod movement;
mod setup;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(attack::Attack)
            .add_systems(Startup, setup::setup)
            .add_systems(Update, movement::movement);
    }
}
