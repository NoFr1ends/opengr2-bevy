use bevy::prelude::*;
use bevy::math::primitives::Plane3d;
use bevy_flycam::PlayerPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use opengr2_bevy::GrannyPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(GrannyPlugin::default())
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, init_scene)
        .run()
}

fn init_scene(
    server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let handle = server.load("test2.gr2#default");

    commands.spawn(SceneBundle {
        scene: handle,
        ..default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(0.0, -3.0, 0.0),
        mesh: meshes.add(Mesh::from(Plane3d { normal: Direction3d::from_xyz(20.0, 0.0, 0.0).unwrap() } )),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });
}
