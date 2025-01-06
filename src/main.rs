use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;

mod player;
mod camera;
mod world;

use player::PlayerPlungin;
use camera::CameraPlungin;
use world::WorldPlungin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlungin, 
            CameraPlungin, 
            WorldPlungin,
            ThirdPersonCameraPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .run();
}
