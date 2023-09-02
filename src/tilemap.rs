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

    //commands.spawn(PbrBundle {
    //    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //    material: materials.add(Color::WHITE.into()),
    //    transform: Transform::from_xyz(-0.9, 0.5, -3.1),
    //    ..default()
    //});

    let f = || {(0,1)};
    // Need to add different textures to walls so it will less boring
    let mut map = vec![
        vec![f(), f(),f(),f(),f(),f(),f()],
        vec![f(), f(),f(),f(),f(),f(),f()],
        vec![f(), f(),f(),f(),f(),f(),f()],
        vec![f(), f(),f(),f(),f(),f(),f()],
        vec![f(), f(),f(),f(),f(),f(),f()],
        vec![f(), f(),f(),f(),f(),f(),f()],
    ];
    //Creates a padding
    //map.insert(0, vec![(0,0); map[0].len()]);
    //map.push(vec![(0,0); map[0].len()]);
    //for row in map.iter_mut() {
    //    row.insert(0, (0,0));
    //    row.push((0,0));
    //}

    for y in 0..map.len(){
        for x in 0..map[y].len(){
            let index = map[y][x].0 + map[y][x].1 + 47;
            println!("{}", index);
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