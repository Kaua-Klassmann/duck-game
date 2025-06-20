use avian2d::prelude::*;
use bevy::prelude::*;

pub(super) struct PhysicPlugin;

impl Plugin for PhysicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default());
    }
}
