use bevy::prelude::*;

use crate::components::RankTimerText;

pub(super) fn setup(mut commands: Commands) {
    commands.spawn((
        Transform::from_xyz(0., 150., 2.),
        Sprite {
            color: Color::srgb(200., 200., 200.),
            custom_size: Some(Vec2 { x: 640., y: 60. }),
            ..Default::default()
        },
    ));

    commands
        .spawn((Node {
            width: Val::Vw(100.),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },))
        .with_child((
            Text::default(),
            TextFont::from_font_size(30.),
            TextColor(Color::BLACK),
            ZIndex(1),
            RankTimerText,
        ));
}
