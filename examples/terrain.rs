use bevy::prelude::*;
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use rustpg::{
    core::{camera::CameraPlugin, spectator::SpectatorPlugin},
    nycthemeron::{time_of_day::TimeOfDay, NycthemeronPlugin},
    terragen::{PlayerPositionChangedEvent, TerragenPlugin},
};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(CameraPlugin)
        .add_plugin(NycthemeronPlugin {
            time_of_day: TimeOfDay::new(12f32, 0f32, 0f32, 7000.0),
            inspectors: true,
        })
        .add_plugin(SpectatorPlugin)
        .add_plugin(WorldInspectorPlugin::default())
        .insert_resource(WorldInspectorParams {
            sort_components: true,
            despawnable_entities: true,
            ..default()
        })
        .add_plugin(TerragenPlugin::default())
        .add_system(send_player_pos_events)
        .run();
}

fn send_player_pos_events(
    query: Query<&Transform, With<Camera>>,
    mut events: EventWriter<PlayerPositionChangedEvent>,
) {
    if let Ok(transform) = query.get_single() {
        events.send(PlayerPositionChangedEvent(transform.translation));
    }
}
