use bevy::{prelude::*, transform::{commands, self}, input::keyboard, render::camera::ScalingMode};
use bevy_sprite3d::*;


use player::*;
use components::*;
use debug::*;
use tilemap::*;

mod player;
pub mod components;
mod debug;
mod tilemap;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins
        .set(ImagePlugin::default_nearest())
    )
    .add_plugins(Sprite3dPlugin)
    .add_plugins(DebugPlugin)
    .add_state::<GameState>()
    .add_systems(Startup, 
        |asset_server: Res<AssetServer>, mut assets: ResMut<ImageAssets>, mut texture_atlases: ResMut<Assets<TextureAtlas>>|{
            assets.image = asset_server.load("sprites/player/spritesheet.png");
            assets.tileset = texture_atlases.add(
                TextureAtlas::from_grid(assets.image.clone(),
                Vec2::new(32.0, 32.0),
                5,
                10,
                None,
                None)
            );
        }
    )
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, spawn_player.run_if(in_state(GameState::Loading)))
    .add_systems(OnEnter(GameState::Ready), spawn_map)
    .add_systems(Update, animate_sprite.run_if(in_state(GameState::Ready)))
    .add_systems(Update, player_movement.run_if(in_state(GameState::Ready)))
    .add_systems( Update, face_camera.run_if(in_state(GameState::Ready)) )
    .insert_resource(ImageAssets::default())
    .run()
}


//animating player sprite
fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut Animation, &mut AtlasSprite3dComponent)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for ( mut animation, mut sprite) in query.iter_mut() {
        //Timer causes sprites to update too late, so it causing bad animation switch
        //TODO: Fix animation switch
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            if (keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up)){
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
                sprite.index = animation.frames[animation.counter]+10;
                animation.counter += 1;
                animation.counter %= animation.frames.len();
            }
            if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right){
                sprite.index = animation.frames[animation.counter]+15;
                animation.counter += 1;
                animation.counter %= animation.frames.len();
            }
            if (keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) && keyboard_input.any_pressed([KeyCode::W, KeyCode::Up])) && !keyboard_input.any_just_released([KeyCode::W, KeyCode::Up, KeyCode::D, KeyCode::Right]){
                    sprite.index = animation.frames[animation.counter]+40;
                    animation.counter += 1;
                    animation.counter %= animation.frames.len();
            }
            if (keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) && keyboard_input.any_pressed([KeyCode::S, KeyCode::Down])) && !keyboard_input.any_just_released([KeyCode::S, KeyCode::Down, KeyCode::D, KeyCode::Right]){
                sprite.index = animation.frames[animation.counter]+30;
                animation.counter += 1;
                animation.counter %= animation.frames.len();
            }
            if (keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) && keyboard_input.any_pressed([KeyCode::W, KeyCode::Up])) && !keyboard_input.any_just_released([KeyCode::W, KeyCode::Up, KeyCode::A, KeyCode::Left]){
                sprite.index = animation.frames[animation.counter]+35;
                animation.counter += 1;
                animation.counter %= animation.frames.len();
            }
            if (keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) && keyboard_input.any_pressed([KeyCode::S, KeyCode::Down])) && !keyboard_input.any_just_released([KeyCode::S, KeyCode::Down, KeyCode::A, KeyCode::Left]){
                sprite.index = animation.frames[animation.counter]+25;
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

//Spawns camera in center of a main window in 3d to use billboard sprites

pub fn spawn_camera(
    mut commands: Commands,
){
    commands.spawn(Camera3dBundle{
        projection: OrthographicProjection{
            scale: 3.0,
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