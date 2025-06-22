use bevy::prelude::*;

use crate::{components, resources};

pub(super) fn update_timer(
    time: Res<Time<Virtual>>,
    mut timer: ResMut<resources::RankTimer>,
    mut query: Query<&mut Text, With<components::RankTimerText>>,
) {
    timer.seconds += time.delta_secs();

    let Ok(mut text) = query.single_mut() else {
        return;
    };

    if timer.seconds >= 60. {
        timer.seconds -= 60.;
        timer.minutes += 1;
    }

    **text = format!("{:02}:{:02}", timer.minutes, timer.seconds as u32);
}
