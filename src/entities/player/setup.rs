use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    collider::CollisionType,
    components::{Player, Velocity},
};

pub(super) fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("player.png");

    commands.spawn((
        Transform::from_xyz(0., 0., 1.),
        Sprite {
            image: texture,
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::rectangle(25., 32.),
        CollisionType::Player,
        CollisionLayers::new(CollisionType::Player, CollisionType::Wall),
        LinearVelocity(Vec2::X),
        LockedAxes::ROTATION_LOCKED,
        Friction::new(0.),
        Player,
        Velocity(Vec2 { x: 128., y: 128. }),
    ));
}
