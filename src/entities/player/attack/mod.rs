use bevy::prelude::*;

mod despawn;
mod setup;

pub(super) struct Attack;

impl Plugin for Attack {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup::setup)
            .add_systems(Update, despawn::despawn);
    }
}
