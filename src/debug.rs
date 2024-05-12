//! Debug menus.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui::Checkbox;
use bevy_egui::{egui, EguiContexts};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::map::config::MapConfig;
use crate::map::{ChunkBorderState, ChunkReloadEvent};

pub fn debug_plugin(app: &mut App) {
    app.add_plugins(WorldInspectorPlugin::new())
        .add_systems(PostUpdate, debug_menu);
}

fn debug_menu(
    mut contexts: EguiContexts,
    mut ev_chunk_reload: EventWriter<ChunkReloadEvent>,
    mut map_config: ResMut<MapConfig>,
    mut next_chunk_borders_state: ResMut<NextState<ChunkBorderState>>,
    window: Query<&Window, With<PrimaryWindow>>,
    chunk_borders_state: Res<State<ChunkBorderState>>,
) {
    // Avoids the nasty poison error when theres no window.
    if window.is_empty() {
        return;
    }

    egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        if ui
            .add(egui::Slider::new(&mut map_config.tile_size, 1.0..=100.0).text("Tile Size"))
            .changed()
        {
            ev_chunk_reload.send(ChunkReloadEvent);
        };
        if ui
            .add(egui::Slider::new(&mut map_config.chunk_tile_count, 5..=100).text("Chunk Size"))
            .changed()
        {
            ev_chunk_reload.send(ChunkReloadEvent);
        };
        let original_state = (*chunk_borders_state.get()).into();
        let mut chunk_borders = original_state;
        ui.add(Checkbox::new(&mut chunk_borders, "Chunk Borders"));

        if original_state != chunk_borders {
            next_chunk_borders_state.set(chunk_borders_state.next());
            ev_chunk_reload.send(ChunkReloadEvent);
        }
    });
}
