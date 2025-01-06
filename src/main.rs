use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;
use iyes_perf_ui::prelude::*;

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
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)

        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        PerfUiRoot {
            display_labels: false,
            layout_horizontal: true,
            ..default()
        },
        PerfUiEntryFPSWorst::default(),
        PerfUiEntryFPS::default(),
    ));
}