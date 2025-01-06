use bevy::prelude::*;
use bevy_third_person_camera::*;

pub struct CameraPlungin;


impl Plugin for CameraPlungin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ThirdPersonCamera {
            aim_enabled: true,
            aim_speed: 3.0, // default
            aim_zoom: 0.7,  // default
            offset_enabled: true,
            offset_toggle_enabled: true,
            gamepad_settings: CustomGamepadSettings { ..default() },
            zoom_enabled: true,        // default
            zoom: Zoom::new(7.0, 13.0), // default
            ..default()
        },
    );
    commands.spawn(camera);
}