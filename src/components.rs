use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerAttack;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct RankTimerText;
