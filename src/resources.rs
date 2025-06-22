use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct RankTimer {
    pub seconds: f32,
    pub minutes: u32,
}
