use bevy::{prelude::*, asset::LoadState};
use crate::components::*;
use bevy_sprite3d::*;
use bevy::utils::Duration;


pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<ImageAssets>,
    mut next_state: ResMut<NextState<GameState>>,
    mut sprite_params: Sprite3dParams,
){
    if asset_server.get_load_state(assets.image.clone()) != LoadState::Loaded{return;}
    next_state.set(GameState::Ready);
    
    let mut entity = |(x, y), tile_x, tile_y, height, frames| {
        let mut timer = Timer::from_seconds(0.2, TimerMode::Repeating);
        timer.set_elapsed(Duration::from_secs_f32(0.2));

        for i in 0usize..height {
            let mut c = commands.spawn((AtlasSprite3d {
                    atlas: assets.tileset.clone(),
                    pixels_per_metre: 16.,
                    index: (tile_x + (tile_y - i)) as usize,
                    transform: Transform::from_xyz(x as f32, i as f32 + 0.498, y),
                    ..default()
                }.bundle(&mut sprite_params),
                Player {},
                FaceCamera{},
            ));

            if frames > 1 {
                c.insert(Animation {
                    frames: (0..frames).map(|j| j + tile_x + (tile_y - i) as usize).collect(),
                    counter: 0,
                    timer: timer.clone(),
                });
            }
        }
    };
    entity((0., 0.), 1, 0, 1, 4);
}

pub const PLAYER_SPEED: f32 = 2.0;

//Defining Player movement inputs, moving player

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>
){
    for mut player_transform in player_query.iter_mut(){
        let cam = match camera_query.get_single(){
            Ok(c) => c,
            Err(e) => Err(format!("Error with loading query: {}", e)).unwrap(),
        };
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A){
            direction += cam.left();
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D){
            direction += cam.right();
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W){
            direction += cam.forward();
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S){
            direction += cam.back();
        }
        direction.y = 0.0;
        let movement =direction.normalize_or_zero()*2.0*time.delta_seconds();
        player_transform.translation += movement;
    }
    //if let Ok(mut transform) = player_query.get_single_mut(){
    //    let mut direction = Vec3::ZERO;
    //    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A){
    //        direction += Vec3::new(-1.0, 0.0, 0.0);
    //    }
    //    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D){
    //        direction += Vec3::new(1.0, 0.0, 0.0);
    //    }
    //    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W){
    //        direction += Vec3::new(-1.0, 0.0, -1.0);
    //    }
    //    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S){
    //        direction += Vec3::new(1.0, 0.0, 1.0);
    //    }
    //    if direction.length() > 0.0 {
    //        direction = direction.normalize();
    //    }
    //    transform.translation += direction*PLAYER_SPEED*time.delta_seconds();
    //}
}