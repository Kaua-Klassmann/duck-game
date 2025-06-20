use bevy::prelude::*;

mod player;
mod wall;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(player::PlayerPlugin)
            .add_plugins(wall::WallPlugin);
    }
}
