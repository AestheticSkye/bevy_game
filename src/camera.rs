//! Camera and zoom functionality.

use bevy::prelude::*;

use crate::get_single_mut;

pub fn camera_plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, update_zoom);
}

fn setup(mut commands: Commands) { commands.spawn(Camera2dBundle::default()); }

fn update_zoom(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut projection: Query<&mut OrthographicProjection, With<Camera>>,
) {
    let mut projection = get_single_mut!(projection);

    if projection.scale >= 0.25 && keyboard_input.pressed(KeyCode::Equal) {
        projection.scale -= 1.0 * time.delta_seconds();
    }

    if projection.scale <= 3.0 && keyboard_input.pressed(KeyCode::Minus) {
        projection.scale += 1.0 * time.delta_seconds();
    }
}
