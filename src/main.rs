use bevy::{prelude::*, transform::commands, window::PrimaryWindow};
fn main() {
    App::new().add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())).add_systems(Startup, (spawn_player, spawn_camera)).add_systems(Update, animate_sprite).run()
}

//Animating sprites
#[derive(Component)]
//Declaring how much images animation will use
pub struct AnimationIndices{
    first: usize,
    last: usize,
}

#[derive(Component)]
pub struct Player{}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);
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
// TODO Optimize the function for loading different entities, not only a main character
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    
    let texture_handle: Handle<Image> = asset_server.load("sprites/player-down-left-idle.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle,Vec2::new(32.0, 32.0),3,1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices{first:1, last:2,};
    commands.spawn((
        SpriteSheetBundle{
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        }, animation_indices, AnimationTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),Player{},)
    );
    
    println!("Player has spawned")
}
//Spawns camera in center of a main window by default
pub fn spawn_camera(
    mut commands: Commands,
){
    commands.spawn(Camera2dBundle::default());
    println!("Camera has spawned")
}