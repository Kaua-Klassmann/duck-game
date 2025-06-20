use bevy::prelude::*;

mod setup;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup::setup);
    }
}
