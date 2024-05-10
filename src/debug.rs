use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::map::config::MapConfig;
use crate::map::ChunkReloadEvent;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::new())
            .add_systems(Update, debug_menu);
    }
}

fn debug_menu(
    mut contexts: EguiContexts,
    mut ev_chunk_reload: EventWriter<ChunkReloadEvent>,
    mut map_config: ResMut<MapConfig>,
) {
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
    });
}
