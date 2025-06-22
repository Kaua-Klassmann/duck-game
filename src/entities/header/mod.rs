use bevy::prelude::*;

use crate::{resources, state};

mod reset_timer;
mod setup;
mod update_timer;

pub struct HeaderPlugin;

impl Plugin for HeaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(resources::RankTimer::default())
            .add_systems(Startup, setup::setup)
            .add_systems(OnEnter(state::AppState::InGame), reset_timer::reset_timer)
            .add_systems(
                Update,
                update_timer::update_timer.run_if(in_state(state::AppState::InGame)),
            );
    }
}
