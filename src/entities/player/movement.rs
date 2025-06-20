use avian2d::prelude::*;
use bevy::prelude::*;

use crate::components::{Player, Velocity};

pub fn movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut velocities: Query<(&mut LinearVelocity, &Velocity), With<Player>>,
) {
    let Ok(mut velocity) = velocities.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    direction = direction.normalize_or_zero();

    velocity.0.x = direction.x * velocity.1.0.x;
    velocity.0.y = direction.y * velocity.1.0.y;
}
