#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::module_name_repetitions)]

mod camera;
mod debug;
mod map;
mod player;
mod util;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
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
            CameraPlugin,
            DebugPlugin,
        ))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
