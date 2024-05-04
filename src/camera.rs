#![allow(clippy::neg_cmp_op_on_partial_ord)]

use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_zoom);
    }
}

fn setup(mut commands: Commands) { commands.spawn(Camera2dBundle::default()); }

fn update_zoom(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut projection: Query<&mut OrthographicProjection, With<Camera>>,
) {
    let Ok(mut projection) = projection.get_single_mut() else {
        return;
    };

    if projection.scale >= 0.25 && keyboard_input.pressed(KeyCode::Equal) {
        projection.scale -= 1.0 * time.delta_seconds();
    }
    if projection.scale <= 3.0 && keyboard_input.pressed(KeyCode::Minus) {
        projection.scale += 1.0 * time.delta_seconds();
    }
}
