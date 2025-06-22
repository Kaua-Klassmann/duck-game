use bevy::{asset::AssetMetaCheck, prelude::*, window};

mod collider;
mod components;
mod configs;
mod entities;
mod resources;
mod state;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(window::WindowPlugin {
                    primary_window: Some(Window {
                        resolution: window::WindowResolution::new(640., 360.),
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_state::<state::AppState>()
        .add_plugins(configs::ConfigsPlugin)
        .add_plugins(entities::EntitiesPlugin)
        .run();
}
