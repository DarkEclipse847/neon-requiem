use bevy::{prelude::*, render::camera::ScalingMode};
use crate::components::{FaceCamera, Player};
//Spawns camera in center of a main window in 3d to use billboard sprites

pub fn spawn_camera(
    mut commands: Commands,
){
    commands.spawn(Camera3dBundle{
        projection: OrthographicProjection{
            scale: 6.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    println!("Camera has spawned");
}


pub fn face_camera(
    camera_query: Query<&Transform, With<Camera3d>>,
    mut query: Query<&mut Transform, (With<FaceCamera>, Without<Camera3d>)>
){
    let camera_transform = camera_query.single();
    for mut transform in query.iter_mut() {
        let mut delta = camera_transform.translation - transform.translation;
        delta.y = 0.0;
        delta += transform.translation;
        transform.look_at(delta, Vec3::Y)
    }
}

//Makes camera follow player, taking his position and creating loop that moves camera into that position
//TODO: It makes the face_camera() function useless somehow, FIX IT
pub fn camera_movement(
    player_query: Query<(&Player, &Transform)>,
    mut camera_query: Query<&mut Transform,(With<Camera3d>, Without<Player>)>,
){
    for (player,player_transform) in player_query.iter(){
        
        let position = player_transform.translation;
        for mut cam_transform in camera_query.iter_mut(){
            cam_transform.translation.x = position.x;
            cam_transform.translation.y = position.y + 2.0;
            cam_transform.translation.z = position.z;
        }
    }
}