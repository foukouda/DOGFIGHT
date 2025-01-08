use bevy::prelude::*;

pub struct WorldPlungin;


impl Plugin for WorldPlungin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_world);
    }
}

fn spawn_world(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    let floor = (
        SceneRoot(assets.load("map.glb#Scene0")),
    );

    let light = (
        PointLight {
            intensity: 150000000000000000.0 * 1000000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 600.0, 60.0),
    );

    commands.spawn(floor);
    commands.spawn(light);
}