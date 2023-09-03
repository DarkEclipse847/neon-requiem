use bevy::prelude::*;
use crate::components::*;
use bevy_sprite3d::*;


pub fn spawn_map(
    mut commands: Commands,
    assets: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams,
    //mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<StandardMaterial>>,
){

    let f = || {(0,1)};
    // Need to add different textures to walls so it will less boring
    let map = vec![
        vec![f(), f(),f(),f(),f(),f(),f()],
        vec![f(), f(),f(),f(),f(),f(),f()],
        vec![f(), f(),f(),f(),f(),f(),f()],
        vec![f(), f(),f(),f(),f(),f(),f()],
        vec![f(), f(),f(),f(),f(),f(),f()],
        vec![f(), f(),f(),f(),f(),f(),f()],
    ];

    for y in 0..map.len(){
        for x in 0..map[y].len(){
            let index = map[y][x].0 + map[y][x].1 + 47;
            let (x, y) = (x as f32 - map[y].len() as f32 / 2.0, y as f32 - map.len() as f32 / 2.0);
            if index == 0 { continue; }
            commands.spawn(AtlasSprite3d {
                atlas: assets.tileset.clone(),
                pixels_per_metre: 16.,
                index: index as usize,
                double_sided: false,
                transform: Transform::from_xyz(x, 0.0 - 1.0, y).with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.0)),
                ..default()
        }.bundle(&mut sprite_params));
        }
    }
}