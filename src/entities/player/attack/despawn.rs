use bevy::prelude::*;

use crate::components;

pub(super) fn despawn(
    mut commands: Commands,
    attacks: Query<(Entity, &Transform), With<components::PlayerAttack>>,
) {
    for (entity, transform) in attacks {
        if transform.translation.x < -336.
            || transform.translation.x > 336.
            || transform.translation.y < -196.
            || transform.translation.y > 146.
        {
            commands.entity(entity).despawn();
        }
    }
}
