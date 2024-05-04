mod map;
mod player;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use map::MapPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    filter:            "info,wgpu_core=warn,wgpu_hal=warn,bevy_game=debug".into(),
                    level:             bevy::log::Level::DEBUG,
                    update_subscriber: None,
                })
                .set(ImagePlugin::default_nearest()),
            MapPlugin,
            PlayerPlugin,
        ))
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
