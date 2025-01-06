use bevy::prelude::*;

pub struct WorldPlungin;


impl Plugin for WorldPlungin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_world);
    }
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,

    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = (
        Mesh3d(meshes.add(Mesh::from(Plane3d::default().mesh().size(15.0, 15.0)))),
        MeshMaterial3d(materials.add(Color::srgb(0.11, 0.27, 0.16))),
    );

    let light = (
        PointLight {
            intensity: 1500.0 * 1000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 0.0),
    );

    commands.spawn(floor);
    commands.spawn(light);
}