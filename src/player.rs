use std::f64::consts::PI;

use bevy::{math::vec3, prelude::*, window};
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;
pub struct PlayerPlungin;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed {
    current_speed: f32,
    min_speed: f32,
    max_speed: f32,
    acceleration: f32,
    deceleration: f32,
}

impl Plugin for PlayerPlungin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement_keyboard);
    }
}

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let player = (
        SceneRoot(assets.load("Player.glb#Scene0")),
        RigidBody::Dynamic,
        Collider::capsule_y(1.0, 0.5),
        Transform::from_xyz(0.0, 500.0, 0.0).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            0.0,
            -PI as f32 / 2.0,
            0.0,
        )),
        Damping {
            linear_damping: 5.0,
            ..default()
        },
        ExternalImpulse::default(),
        Player,
        ThirdPersonCameraTarget,
        Speed {
            current_speed: 200.0, // vitesse de départ
            min_speed: 800.0,     // vitesse minimale
            max_speed: 1800.0,    // vitesse maximale
            acceleration: 300.0,  // facteur d’accélération
            deceleration: 150.0,  // facteur de décélération
        },
    );

    commands.spawn(player);
}

fn player_movement_keyboard(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<(&mut ExternalImpulse, &mut Transform, &mut Speed), With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    window: Single<&mut Window>,
) {
    if let Ok(cam) = cam_q.get_single() {
        for (mut ext_impulse, mut player_transform, mut speed) in &mut player_q {
            // 1) Calcul de la direction à partir des touches

            if keys.pressed(KeyCode::KeyW) {
                speed.current_speed += speed.acceleration * time.delta_secs();
                if speed.current_speed > speed.max_speed {
                    speed.current_speed = speed.max_speed;
                } else {
                    // Décélération naturelle jusqu’à min_speed
                    speed.current_speed -= speed.deceleration * time.delta_secs();
                    if speed.current_speed < speed.min_speed {
                        speed.current_speed = speed.min_speed;
                    }
                }
            }

            // 3) Faire pivoter le joueur dans la direction de la **caméra**,
            //    plutôt que seulement la direction de déplacement
            let camera_forward_flat =
                Vec3::new(cam.forward().x, 0.0, cam.forward().z).normalize_or_zero();
            let rotate_quat = Quat::from_rotation_y(-PI as f32 / 2.0);
            // oriente le joueur pour qu’il fasse face à l’axe « devant » de la caméra
            player_transform.look_to(rotate_quat.mul_vec3(camera_forward_flat), Vec3::Y);

            let forward = player_transform.forward();
            let movement = forward * speed.current_speed * time.delta_secs();
            ext_impulse.impulse -= rotate_quat.mul_vec3(movement);

            let pitch_speed = 15.0; // Ajustez cette valeur pour un tangage plus ou moins sensible

            if keys.pressed(KeyCode::KeyA) {
                // Monte (pitch up)
                player_transform.rotate_local_z(pitch_speed * time.delta_secs());
                ext_impulse.impulse -= rotate_quat.mul_vec3(movement + vec3(0.0, 15.0, 0.0));
            }

            if keys.pressed(KeyCode::KeyQ) {
                // Descend (pitch down)
                player_transform.rotate_local_z(-pitch_speed * time.delta_secs());
                ext_impulse.impulse -= rotate_quat.mul_vec3(movement + vec3(0.0, -15.0, 0.0));
            }
            let curso_pos = window.cursor_position();
            if !curso_pos.is_none() {
                let cursor_pos = curso_pos.unwrap().x;
                let width = window.width();
                let calcule = cursor_pos / width - 0.5;
    
                player_transform.rotate_local_x(pitch_speed * calcule* -4.0 * time.delta_secs());
                ext_impulse.impulse -= rotate_quat.mul_vec3(movement + vec3(0.0, 0.0, -60.0*calcule));
            }

        }
    }
}
