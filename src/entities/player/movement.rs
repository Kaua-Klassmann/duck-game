use avian2d::prelude::*;
use bevy::prelude::*;

use crate::components::{Player, Velocity};

pub fn movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut velocities: Query<(&mut LinearVelocity, &Velocity), With<Player>>,
) {
    let Ok((mut linear_velocity, velocity)) = velocities.single_mut() else {
        return;
    };

    if !keyboard_input.any_pressed([
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
        KeyCode::ArrowLeft,
        KeyCode::ArrowRight,
    ]) {
        *linear_velocity = LinearVelocity::ZERO;
        return;
    }

    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    *linear_velocity = LinearVelocity(direction.normalize_or_zero() * velocity.0);
}
