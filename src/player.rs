use bevy::{prelude::*, asset::LoadState};
use crate::components::*;
use bevy_sprite3d::*;


pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<ImageAssets>,
    mut next_state: ResMut<NextState<GameState>>,
    mut sprite_params: Sprite3dParams,
){
    if asset_server.get_load_state(assets.image.clone()) != LoadState::Loaded{return;}
    next_state.set(GameState::Ready);
    commands.spawn((AtlasSprite3d {
        atlas: assets.tileset.clone(),

        pixels_per_metre: 32.,
        partial_alpha: true,
        unlit: true,
        index: 3,
        // transform: Transform::from_xyz(0., 0., 0.),
        // pivot: Some(Vec2::new(0.5, 0.5)),

        ..default()}
    .bundle(&mut sprite_params), Player{}))
    .insert(AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
}

pub const PLAYER_SPEED: f32 = 2.0;

//Defining Player movement inputs, moving player

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
){
    if let Ok(mut transform) = player_query.get_single_mut(){
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A){
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D){
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W){
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S){
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        transform.translation += direction*PLAYER_SPEED*time.delta_seconds();
    }
}