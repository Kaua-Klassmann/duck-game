use bevy::{prelude::*, asset::{AssetPlugin, AssetMetaCheck}, render::camera::ScalingMode, window};

pub(super) struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(window::WindowPlugin {
            primary_window: Some(Window {
                resolution: window::WindowResolution::new(640., 360.),
                title: "Jogo do Pato".into(),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }).set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..Default::default()
        }).set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 360.,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
