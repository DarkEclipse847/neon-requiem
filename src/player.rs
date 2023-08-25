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
    commands.spawn(AtlasSprite3d {
        atlas: assets.tileset.clone(),

        pixels_per_metre: 32.,
        partial_alpha: true,
        unlit: true,
        index: 3,
        // transform: Transform::from_xyz(0., 0., 0.),
        // pivot: Some(Vec2::new(0.5, 0.5)),

        ..default()}
    .bundle(&mut sprite_params))
    .insert(AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
}