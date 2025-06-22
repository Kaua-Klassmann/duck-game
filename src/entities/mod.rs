use bevy::prelude::*;

mod header;
mod player;
mod wall;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(player::PlayerPlugin)
            .add_plugins(wall::WallPlugin)
            .add_plugins(header::HeaderPlugin);
    }
}
