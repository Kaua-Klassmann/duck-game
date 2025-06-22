use bevy::prelude::*;

use crate::resources;

pub(super) fn reset_timer(mut timer: ResMut<resources::RankTimer>) {
    timer.seconds = 0.0;
}
