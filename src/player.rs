use bevy::prelude::*;
use crate::components::*;

// TODO Optimize the function for loading different entities, not only a main character
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,

){

    let texture_handle: Handle<Image> = asset_server.load("sprites/player-down-left-idle.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle,Vec2::new(32.0, 32.0),5,1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices{first:1, last:4,};
    commands.spawn((
        SpriteSheetBundle{
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        }, animation_indices, AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),Player{},)
    );
    println!("Player has spawned");
}


pub fn change_player_animation(
    mut player: Query<(&mut Handle<TextureAtlas>, &mut AnimationIndices, &mut TextureAtlasSprite), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>
){
    let(mut atlas, mut animation, mut sprite) = player.single_mut();

    // Movement UP animation
    if keyboard_input.just_pressed(KeyCode::W) || keyboard_input.just_pressed(KeyCode::Up){
        let new_atlas = TextureAtlas::from_grid(
            asset_server.load("sprites/player-going-up.png"),
            Vec2::splat(32.0),
            5,
            1,
            None,
            None,
        );
        *atlas = texture_atlases.add(new_atlas);
        animation.first = 1;
        animation.last = 4;
        sprite.index = 0;
    }
    // Movement DOWN animation
    if keyboard_input.just_pressed(KeyCode::S) || keyboard_input.just_pressed(KeyCode::Down){
        let new_atlas = TextureAtlas::from_grid(
            asset_server.load("sprites/player-going-down.png"),
            Vec2::splat(32.0),
            5,
            1,
            None,
            None,
        );
        *atlas = texture_atlases.add(new_atlas);
        animation.first = 1;
        animation.last = 4;
        sprite.index = 0;
    }
    // Movement to the LEFT animation
    if keyboard_input.just_pressed(KeyCode::A) || keyboard_input.just_pressed(KeyCode::Left){
        let new_atlas = TextureAtlas::from_grid(
            asset_server.load("sprites/player-going-left.png"),
            Vec2::splat(32.0),
            5,
            1,
            None,
            None,
        );
        *atlas = texture_atlases.add(new_atlas);
        animation.first = 1;
        animation.last = 4;
        sprite.index = 0;
    }
    // Movement to the RIGHT animation
    if keyboard_input.just_pressed(KeyCode::D) || keyboard_input.just_pressed(KeyCode::Right){
        let new_atlas = TextureAtlas::from_grid(
            asset_server.load("sprites/player-going-right.png"),
            Vec2::splat(32.0),
            5,
            1,
            None,
            None,
        );
        *atlas = texture_atlases.add(new_atlas);
        animation.first = 1;
        animation.last = 4;
        sprite.index = 0;
    }

    //Diagonal animations

    //Moving UP-RIGHT animation
    if (keyboard_input.any_just_pressed([KeyCode::D, KeyCode::Right]) && keyboard_input.any_just_pressed([KeyCode::W, KeyCode::Up])) || (keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) && keyboard_input.any_just_pressed([KeyCode::W, KeyCode::Up])) || (keyboard_input.any_just_pressed([KeyCode::D, KeyCode::Right]) && keyboard_input.any_pressed([KeyCode::W, KeyCode::Up])){
        let new_atlas = TextureAtlas::from_grid(
            asset_server.load("sprites/player-going-up-right.png"),
            Vec2::splat(32.0),
            5,
            1,
            None,
            None,
        );
        *atlas = texture_atlases.add(new_atlas);
        animation.first = 1;
        animation.last = 4;
        sprite.index = 0;
    }

    //Moving UP-LEFT animation
    if (keyboard_input.any_just_pressed([KeyCode::A, KeyCode::Left]) && keyboard_input.any_just_pressed([KeyCode::W, KeyCode::Up])) || (keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) && keyboard_input.any_just_pressed([KeyCode::W, KeyCode::Up])) || (keyboard_input.any_just_pressed([KeyCode::A, KeyCode::Left]) && keyboard_input.any_pressed([KeyCode::W, KeyCode::Up])){
        let new_atlas = TextureAtlas::from_grid(
            asset_server.load("sprites/player-going-up-left.png"),
            Vec2::splat(32.0),
            5,
            1,
            None,
            None,
        );
        *atlas = texture_atlases.add(new_atlas);
        animation.first = 1;
        animation.last = 4;
        sprite.index = 0;
    }

    //Moving DOWN-LEFT animation
    if (keyboard_input.any_just_pressed([KeyCode::A, KeyCode::Left]) && keyboard_input.any_just_pressed([KeyCode::S, KeyCode::Down])) || (keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) && keyboard_input.any_just_pressed([KeyCode::S, KeyCode::Down])) || (keyboard_input.any_just_pressed([KeyCode::A, KeyCode::Left]) && keyboard_input.any_pressed([KeyCode::S, KeyCode::Down])){
        let new_atlas = TextureAtlas::from_grid(
            asset_server.load("sprites/player-going-down-left.png"),
            Vec2::splat(32.0),
            5,
            1,
            None,
            None,
        );
        *atlas = texture_atlases.add(new_atlas);
        animation.first = 1;
        animation.last = 4;
        sprite.index = 0;
    }
    
    //Moving DOWN-RIGHT animation
    if (keyboard_input.any_just_pressed([KeyCode::D, KeyCode::Right]) && keyboard_input.any_just_pressed([KeyCode::S, KeyCode::Down])) || (keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) && keyboard_input.any_just_pressed([KeyCode::S, KeyCode::Down])) || (keyboard_input.any_just_pressed([KeyCode::D, KeyCode::Right]) && keyboard_input.any_pressed([KeyCode::S, KeyCode::Down])){
        let new_atlas = TextureAtlas::from_grid(
            asset_server.load("sprites/player-going-down-right.png"),
            Vec2::splat(32.0),
            5,
            1,
            None,
            None,
        );
        *atlas = texture_atlases.add(new_atlas);
        animation.first = 1;
        animation.last = 4;
        sprite.index = 0;
    }
    //Returning to idle state
    if keyboard_input.any_just_released([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right, KeyCode::W, KeyCode::Up, KeyCode::S, KeyCode::Down]) && !keyboard_input.any_pressed([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right, KeyCode::W, KeyCode::Up, KeyCode::S, KeyCode::Down]){
        let new_atlas = TextureAtlas::from_grid(
            asset_server.load("sprites/player-down-left-idle.png"),
            Vec2::splat(32.0),
            5,
            1,
            None,
            None,
        );
        *atlas = texture_atlases.add(new_atlas);
        animation.first = 1;
        animation.last = 4;
        sprite.index = 0;
    }


    if keyboard_input.any_just_released([KeyCode::D, KeyCode::Right, KeyCode::A, KeyCode::Left]) && keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]){
        let new_atlas = TextureAtlas::from_grid(
            asset_server.load("sprites/player-going-up.png"),
            Vec2::splat(32.0),
            5,
            1,
            None,
            None,
        );
        *atlas = texture_atlases.add(new_atlas);
        animation.first = 1;
        animation.last = 4;
        sprite.index = 0;
    }
    if keyboard_input.any_just_released([KeyCode::W, KeyCode::Up, KeyCode::S, KeyCode::Down,]) && keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]){
        let new_atlas = TextureAtlas::from_grid(
            asset_server.load("sprites/player-going-right.png"),
            Vec2::splat(32.0),
            5,
            1,
            None,
            None,
        );
        *atlas = texture_atlases.add(new_atlas);
        animation.first = 1;
        animation.last = 4;
        sprite.index = 0;
    }
    if keyboard_input.any_just_released([KeyCode::W, KeyCode::Up, KeyCode::S, KeyCode::Down,]) && keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]){
        let new_atlas = TextureAtlas::from_grid(
            asset_server.load("sprites/player-going-left.png"),
            Vec2::splat(32.0),
            5,
            1,
            None,
            None,
        );
        *atlas = texture_atlases.add(new_atlas);
        animation.first = 1;
        animation.last = 4;
        sprite.index = 0;
    }
    if keyboard_input.any_just_released([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right,]) && keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]){
        let new_atlas = TextureAtlas::from_grid(
            asset_server.load("sprites/player-going-down.png"),
            Vec2::splat(32.0),
            5,
            1,
            None,
            None,
        );
        *atlas = texture_atlases.add(new_atlas);
        animation.first = 1;
        animation.last = 4;
        sprite.index = 0;
    }

}

pub const PLAYER_SPEED: f32 = 300.0;


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