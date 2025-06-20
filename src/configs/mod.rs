use bevy::prelude::*;

mod physic;
mod window;

pub struct ConfigsPlugin;

impl Plugin for ConfigsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(window::WindowPlugin)
            .add_plugins(physic::PhysicPlugin);
    }
}
