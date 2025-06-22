use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{collider::CollisionType, components};

pub(super) fn setup(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    player: Query<(&LinearVelocity, &Transform), With<components::Player>>,
) {
    if !keyboard_input.just_pressed(KeyCode::KeyE) {
        return;
    }

    let Ok((player_velocity, player_position)) = player.single() else {
        return;
    };

    let linear_velocity = if *player_velocity == LinearVelocity::ZERO {
        Vec2 { x: -512., y: 0. }
    } else {
        player_velocity.normalize() * 512.
    };

    commands.spawn((
        *player_position,
        Sprite {
            image: asset_server.load("rock.png"),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::rectangle(16., 16.),
        CollisionType::PlayerAttack,
        CollisionLayers::new(CollisionType::Player, CollisionType::Enemy),
        components::PlayerAttack,
        LinearVelocity::from(linear_velocity),
        AngularVelocity::from(128.),
    ));
}
