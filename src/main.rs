use bevy::prelude::*;
use bevy_sprite3d::*;
use bevy_rapier3d::prelude::*;


use player::*;
use components::*;
use debug::*;
use tilemap::*;
use camera::*;
use menu::*;

mod player;
pub mod components;
mod menu;
mod debug;
mod tilemap;
mod camera;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins
        .set(ImagePlugin::default_nearest())
    )
    .add_plugins(Sprite3dPlugin)
    .add_plugins(DebugPlugin)
    .add_plugins(RapierDebugRenderPlugin::default())
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_state::<GameState>()
    .add_systems(Startup, 
        |asset_server: Res<AssetServer>, mut assets: ResMut<ImageAssets>, mut texture_atlases: ResMut<Assets<TextureAtlas>>|{
            assets.image = asset_server.load("sprites/player/spritesheet.png");
            assets.tileset = texture_atlases.add(
                TextureAtlas::from_grid(assets.image.clone(),
                Vec2::new(32.0, 32.0),
                5,
                11,
                None,
                None)
            );
        }
    )
    .add_systems(Startup, spawn_camera)
    .add_systems(OnEnter(GameState::MainMenu), menu_setup)
    .add_systems(Update, start_button_clicked.run_if(in_state(GameState::MainMenu)))
    .add_systems(Update, spawn_player.run_if(in_state(GameState::Loading)))
    .add_systems(OnEnter(GameState::Ready), spawn_map)
    .add_systems(Update, animate_sprite.run_if(in_state(GameState::Ready)))
    .add_systems(Update, player_movement.run_if(in_state(GameState::Ready)))
    .add_systems( OnEnter(GameState::Ready), face_camera)
    .add_systems( Update, camera_movement.run_if(in_state(GameState::Ready)))
    //.add_systems(Update, collision_system.run_if(in_state(GameState::Ready)))
    .insert_resource(ImageAssets::default())
    .run()
}