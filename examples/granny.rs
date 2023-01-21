use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use opengr2_bevy::GrannyPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(GrannyPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_startup_system(init_scene)
        .run()
}

fn init_scene(
    server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands
) {
    let handle = server.load("test2.gr2#default");

    commands.spawn(SceneBundle {
        scene: handle,
        ..default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(0.0, -3.0, 0.0),
        mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
}