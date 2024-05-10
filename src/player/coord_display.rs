use bevy::prelude::*;

use super::Player;
use crate::map::config::MapConfig;
use crate::map::position::Position;
use crate::{get_single, get_single_mut};

#[derive(Component)]
pub struct CoordLabel;

pub fn setup_coords(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 25.,
        ..default()
    };

    commands.spawn((
        TextBundle {
            text: Text::from_sections(vec![
                TextSection::new("Chunk Position\n", text_style.clone()),
                TextSection::new("0 0\n", text_style.clone()),
                TextSection::new("World Position\n", text_style.clone()),
                TextSection::new("0 0\n", text_style),
            ]),
            style: Style {
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Px(10.0),
                    top: Val::Px(10.0),
                    ..default()
                },
                justify_items: JustifyItems::End,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK.with_a(0.6)),
            ..default()
        },
        CoordLabel,
    ));
}

pub fn update_coords(
    player_pos: Query<&Transform, With<Player>>,
    mut coord_text: Query<&mut Text, With<CoordLabel>>,
    map_config: Res<MapConfig>,
) {
    let transform = get_single!(player_pos);
    let mut coord_text = get_single_mut!(coord_text);

    let (x, y) = (transform.translation.x, transform.translation.y);

    let chunk_position = Position::from_xy((x, y), &map_config);

    let chunk_position = format!("{} {}\n", chunk_position.x, chunk_position.y);
    coord_text.sections[1].value = chunk_position;

    let world_position = format!(
        "{} {}\n",
        (x / map_config.tile_size) as i32,
        (y / map_config.tile_size) as i32
    );
    coord_text.sections[3].value = world_position;
}
