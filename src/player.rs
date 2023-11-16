use bevy::{prelude::*, asset::LoadState, sprite::collide_aabb::collide, transform};
use crate::components::*;
use bevy_sprite3d::*;
use bevy_rapier3d::prelude::*;
use bevy::utils::Duration;

pub const PLAYER_SIZE: f32 = 1.0;
pub const TILE_SIZE: f32 = 1.0;


pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<ImageAssets>,
    mut next_state: ResMut<NextState<GameState>>,
    mut sprite_params: Sprite3dParams,
){
    //if asset_server.get_load_state(assets.image.clone()) != LoadState::Loaded{return;}
    next_state.set(GameState::Ready);
    
    let mut entity = |(x, y), tile_x, tile_y, height, frames| {
        let mut timer = Timer::from_seconds(0.2, TimerMode::Repeating);
        timer.set_elapsed(Duration::from_secs_f32(0.2));

        for i in 0usize..height {
            let mut c = commands.spawn((AtlasSprite3d {
                    atlas: assets.tileset.clone(),
                    pixels_per_metre: 16.,
                    index: (tile_x + (tile_y - i)) as usize,
                    transform: Transform::from_xyz(x as f32, i as f32, y),
                    ..default()
                }.bundle(&mut sprite_params),
                Player {},
                FaceCamera{},
                Collider::cuboid(0.5 ,1.0, 0.5),
                RigidBody::Dynamic,
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
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>, Without<Camera3d>)>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    //wall_query: Query<&Transform, (With<Wall>, Without<Player>)>,
    camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>
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
        let movement = direction.normalize_or_zero()*PLAYER_SPEED*time.delta_seconds();
        player_transform.translation += movement;

    }
}


//animating player sprite
//TODO: Need to see how it will behave with another entities(npcs), because we do not wire this animation with player as i can tell....
pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut Animation, &mut AtlasSprite3dComponent)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for ( mut animation, mut sprite) in query.iter_mut() {
        //Timer causes sprites to update too late, so it causing bad animation switch
        //TODO: Fix animation switch
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up){
                sprite.index = animation.frames[animation.counter]+20;
                animation.counter += 1;
                animation.counter %= animation.frames.len();
            }
            if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
                sprite.index = animation.frames[animation.counter]+5;
                animation.counter += 1;
                animation.counter %= animation.frames.len();
            }
            if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
                sprite.index = animation.frames[animation.counter]+15;
                animation.counter += 1;
                animation.counter %= animation.frames.len();
            }
            if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right){
                sprite.index = animation.frames[animation.counter]+10;
                animation.counter += 1;
                animation.counter %= animation.frames.len();
            }
            if (keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) && keyboard_input.any_pressed([KeyCode::W, KeyCode::Up])) && !keyboard_input.any_just_released([KeyCode::W, KeyCode::Up, KeyCode::D, KeyCode::Right]){
                    sprite.index = animation.frames[animation.counter]+35;
                    animation.counter += 1;
                    animation.counter %= animation.frames.len();
            }
            if (keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) && keyboard_input.any_pressed([KeyCode::S, KeyCode::Down])) && !keyboard_input.any_just_released([KeyCode::S, KeyCode::Down, KeyCode::D, KeyCode::Right]){
                sprite.index = animation.frames[animation.counter]+25;
                animation.counter += 1;
                animation.counter %= animation.frames.len();
            }
            if (keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) && keyboard_input.any_pressed([KeyCode::W, KeyCode::Up])) && !keyboard_input.any_just_released([KeyCode::W, KeyCode::Up, KeyCode::A, KeyCode::Left]){
                sprite.index = animation.frames[animation.counter]+40;
                animation.counter += 1;
                animation.counter %= animation.frames.len();
            }
            if (keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) && keyboard_input.any_pressed([KeyCode::S, KeyCode::Down])) && !keyboard_input.any_just_released([KeyCode::S, KeyCode::Down, KeyCode::A, KeyCode::Left]){
                sprite.index = animation.frames[animation.counter]+30;
                animation.counter += 1;
                animation.counter %= animation.frames.len();
            }
            if !keyboard_input.any_pressed([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right, KeyCode::W, KeyCode::Up, KeyCode::S, KeyCode::Down]){
                sprite.index = animation.frames[animation.counter];
                animation.counter += 1;
                animation.counter %= animation.frames.len();
            }
        }
    }
}