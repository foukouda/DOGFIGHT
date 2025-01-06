use std::f64::consts::PI;

use bevy::prelude::*;
use bevy_third_person_camera::*;
use bevy_rapier3d::prelude::*;
pub struct PlayerPlungin;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

impl Plugin for PlayerPlungin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_player)
        .add_systems(Update, player_movement_keyboard);
    }
}

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let player = (
        SceneRoot(assets.load("Player.glb#Scene0")),
        Transform::from_xyz(0.0, 500.0, 0.0).with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0,  -PI as f32 / 2.0,0.0)),
        Damping {
            linear_damping: 5.0,
            ..default()
        },
        ExternalImpulse::default(),
        Player,
        ThirdPersonCameraTarget,
        Speed(4.0),
    );



    commands.spawn(player);
}

fn player_movement_keyboard(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<(&mut ExternalImpulse, &mut Transform, &Speed), With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    if let Ok(cam) = cam_q.get_single() {
        for (mut ext_impulse, mut player_transform, player_speed) in &mut player_q {
            // 1) Calcul de la direction à partir des touches
            let mut direction = Vec3::ZERO;

            if keys.pressed(KeyCode::KeyW) {
                direction.x += 1.0;
            }
            if keys.pressed(KeyCode::KeyS) {
                direction.x -= 1.0;
            }
            if keys.pressed(KeyCode::KeyA) {
                direction.y += 1.0;
            }
            if keys.pressed(KeyCode::KeyD) {
                direction.y -= 1.0;
            }

            direction.y = 0.0;

            let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_secs();
            ext_impulse.impulse += movement;

            // 3) Faire pivoter le joueur dans la direction de la **caméra**, 
            //    plutôt que seulement la direction de déplacement
            let camera_forward_flat = Vec3::new(cam.forward().x, 0.0, cam.forward().z).normalize_or_zero();
            let rotate_quat = Quat::from_rotation_y(-PI as f32 / 2.0);
            // oriente le joueur pour qu’il fasse face à l’axe « devant » de la caméra
            player_transform.look_to(rotate_quat.mul_vec3(camera_forward_flat) , Vec3::Y);
        }
    }
}
