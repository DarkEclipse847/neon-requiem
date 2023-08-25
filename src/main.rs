use bevy::{prelude::*, transform::{commands, self}, window::PrimaryWindow, sprite, asset::ChangeWatcher, render::camera::ScalingMode};
use bevy_ecs_tilemap::prelude::*;
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
            assets.image = asset_server.load("sprites/player/player-down-left-idle.png");
            assets.tileset = texture_atlases.add(
                TextureAtlas::from_grid(assets.image.clone(),
                Vec2::new(32.0, 32.0),
                5,
                1,
                None,
                None)
            ); 
        }
    )
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, spawn_player.run_if(in_state(GameState::Loading)))
    .add_systems(Update, animate_sprite.run_if(in_state(GameState::Ready)))
    .insert_resource(ImageAssets::default())
    .run()
}


//Animation fuction which creates a loop through sprites using timer
fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut AtlasSprite3dComponent)>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = (sprite.index + 1) % sprite.atlas.len();
        }
    }
}


//Spawns camera in center of a main window in 3d
//to use billboard sprites which i hopefully add later using bevy_sprites3d

pub fn spawn_camera(
    mut commands: Commands,
){
    commands.spawn(Camera3dBundle::default()).insert(Transform::from_xyz(0.0, 0.0, 5.0));
    println!("Camera has spawned");
}