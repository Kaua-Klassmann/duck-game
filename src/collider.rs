use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(PhysicsLayer, Default, Component)]
pub enum CollisionType {
    #[default]
    Player,
    PlayerAttack,
    Enemy,
    Wall,
}
