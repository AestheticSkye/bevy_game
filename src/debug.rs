//! Debug menus.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui::Checkbox;
use bevy_egui::{egui, EguiContexts};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::map::config::MapConfig;
use crate::map::{ChunkBorderState, ChunkReloadEvent};

pub fn debug_plugin(app: &mut App) {
    app.init_resource::<DebugState>()
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(PostUpdate, debug_menu);
}

#[derive(Resource, Default)]
struct DebugState {
    seed_text: String,
}

fn debug_menu(
    mut contexts: EguiContexts,
    mut ev_chunk_reload: EventWriter<ChunkReloadEvent>,
    mut map_config: ResMut<MapConfig>,
    mut debug_state: ResMut<DebugState>,
    mut next_chunk_borders_state: ResMut<NextState<ChunkBorderState>>,
    window: Query<&Window, With<PrimaryWindow>>,
    chunk_borders_state: Res<State<ChunkBorderState>>,
) {
    // Avoids the nasty poison error when theres no window.
    if window.is_empty() {
        return;
    }

    egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        let textfield_response = ui.text_edit_singleline(&mut debug_state.seed_text);
        if textfield_response
            .ctx
            .input(|i| i.key_pressed(egui::Key::Enter))
        {
            let hex_string = &sha256::digest(&debug_state.seed_text)[0..16];
            let seed = u64::from_str_radix(hex_string, 16).expect("Failed to parse seed");
            map_config.seed = seed;
            ev_chunk_reload.send(ChunkReloadEvent);
        };

        ui.horizontal(|ui| {
            if ui.button("Random Seed").clicked() {
                map_config.seed = rand::random::<u64>();
                debug_state.seed_text.clear();
                ev_chunk_reload.send(ChunkReloadEvent);
            }
            if ui.button("Copy Seed").clicked() {
                ui.output_mut(|o| o.copied_text = map_config.seed.to_string());
            }
        });

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

        let mut chunk_borders = (*chunk_borders_state.get()).into();
        if ui
            .add(Checkbox::new(&mut chunk_borders, "Chunk Borders"))
            .changed()
        {
            next_chunk_borders_state.set(chunk_borders_state.next());
            ev_chunk_reload.send(ChunkReloadEvent);
        }
    });
}
