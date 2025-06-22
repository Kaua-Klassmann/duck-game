use avian2d::prelude::*;
use bevy::prelude::*;

use crate::collider::CollisionType;

pub(super) fn setup(mut commands: Commands) {
    // TOP WALL
    commands.spawn((
        Transform::from_xyz(0., 120., 1.),
        RigidBody::Static,
        Collider::rectangle(640., 1.),
        CollisionType::Wall,
        CollisionLayers::new(CollisionType::Wall, CollisionType::Player),
    ));

    // BOTTOM WALL
    commands.spawn((
        Transform::from_xyz(0., -181., 1.),
        RigidBody::Static,
        Collider::rectangle(640., 1.),
        CollisionType::Wall,
        CollisionLayers::new(CollisionType::Wall, CollisionType::Player),
    ));

    // RIGHT WALL
    commands.spawn((
        Transform::from_xyz(320., 0., 1.),
        RigidBody::Static,
        Collider::rectangle(1., 360.),
        CollisionType::Wall,
        CollisionLayers::new(CollisionType::Wall, CollisionType::Player),
    ));

    // LEFT WALL
    commands.spawn((
        Transform::from_xyz(-321., 0., 1.),
        RigidBody::Static,
        Collider::rectangle(1., 360.),
        CollisionType::Wall,
        CollisionLayers::new(CollisionType::Wall, CollisionType::Player),
    ));
}
