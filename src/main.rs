use bevy::{prelude::*, transform::{commands, self}, window::PrimaryWindow, sprite};
use player::*;
use components::*;

mod player;
pub mod components;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_systems(Startup, (spawn_player, spawn_camera))
    .add_systems(Update, (animate_sprite, player_movement, change_player_animation))
    .run()
}
//Animating sprites

//Animation fuction which creates a loop through sprites using timer
pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,

){
    for(indices, mut timer, mut sprite)in &mut query{
        timer.tick(time.delta());
        if timer.just_finished(){
            sprite.index = if sprite.index == indices.last{
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}


//Spawns camera in center of a main window by default
pub fn spawn_camera(
    mut commands: Commands,
){
    commands.spawn(Camera2dBundle::default());
    println!("Camera has spawned");
}