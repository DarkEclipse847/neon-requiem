use bevy::prelude::*;
use crate::components::*;
use bevy_sprite3d::*;
use bevy_rapier3d::prelude::*;


pub fn spawn_map(
    mut commands: Commands,
    assets: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams,
    //mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<StandardMaterial>>,
){

    let f = || {(0,1)};
    // Need to add different textures to walls so it will less boring
    let mut map = vec![
        vec![f(),f(),f(),f(),f(),f()],
        vec![f(),f(),f(),f(),f(),f()],
        vec![f(),f(),f(),f(),f(),f()],
        vec![f(),f(),f(),f(),f(),f()],
        vec![f(),f(),f(),f(),f(),f()],
        vec![f(),f(),f(),f(),f(),f()],
    ];


    //Creates a padding
    map.insert(0, vec![(4,0); map[0].len()]);
    map.push(vec![(4,0); map[0].len()]);
    for row in map.iter_mut() {
        row.insert(0, (4,0));
        row.push((4,0));
    }

    //Adds floor tiles
    for y in 0..map.len(){
        for x in 0..map[y].len(){
            let index = map[y][x].0 + map[y][x].1 + 47;
            let (x, y) = (x as f32 - map[y].len() as f32 / 2.0, y as f32 - map.len() as f32 / 2.0);
            if index == 0 { continue; }
            commands.spawn((AtlasSprite3d {
                atlas: assets.tileset.clone(),
                pixels_per_metre: 31.9,
                index: index as usize,
                double_sided: false,
                transform: Transform::from_xyz(x, 0.0 - 1.0, y).with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.0)),
                ..default()
        }.bundle(&mut sprite_params),
        Collider::cuboid(0.5, 0.5, 0.01),
        RigidBody::Fixed,
        ));
        }
    }

    //Adds walls

    //horisontal walls
    for y in 1..(map.len()-1){
        for x in 0..(map[y].len()-1){
            if (map[y][x] != (4,0)) ^ (map[y][x+1] == (4,0)) {
                continue;
            }
            println!("{}, {}", x, y);
            
            let dir = if map[y][x] == (4,0) { 1.0 } else { -1.0 };
            let (x, y) = (x as f32 - map[y].len() as f32 / 2.0, y as f32 - map.len() as f32 / 2.0);
            for i in [0,1]{ // add bottom and top piece
                commands.spawn((AtlasSprite3d {
                        atlas: assets.tileset.clone(),
                        pixels_per_metre: 32.2,
                        index: 48 as usize,
                        double_sided: false,
                        transform: Transform::from_xyz(x+0.5, i as f32 - 0.499, y)
                            .with_rotation(Quat::from_rotation_y(
                                dir * std::f32::consts::PI / 2.0)),
                        ..default()
                }.bundle(&mut sprite_params),
                Collider::cuboid(0.5, 0.5, 0.01),
                RigidBody::Fixed,
            ));
            }
        }
    }
    //vertical walls
    for x in 1..(map.len()-1){
        for y in 0..(map[x].len()-1){
            if (map[y][x] != (4,0)) ^ (map[y+1][x] == (4,0)) {
                continue;
            }
            let dir = if map[y][x] == (4,0) { 1.0 } else { -1.0 };
            let (x, y) = (x as f32 - map[y].len() as f32 / 2.0, y as f32 - map.len() as f32 / 2.0);
            for i in [0,1]{ // add bottom and top piece
                commands.spawn((AtlasSprite3d {
                        atlas: assets.tileset.clone(),
                        pixels_per_metre: 32.2,
                        index: 48 as usize,
                        double_sided: false,
                        transform: Transform::from_xyz(x, i as f32 - 0.499, y+0.5)
                            .with_rotation(Quat::from_rotation_y(
                                (dir-1.0) * std::f32::consts::PI / 2.0)),
                        ..default()
                }.bundle(&mut sprite_params),
                Collider::cuboid(0.5, 0.5, 0.01),
                RigidBody::Fixed,
                ));
            }
        }
    }

    //Spawn light sourse(test)
    commands.spawn((PointLightBundle {
        point_light: PointLight {
            intensity: 100.0,
            color: Color::VIOLET,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 1.8, -2.5),
        ..default()
    },
    RigidBody::Dynamic,
    Collider::ball(0.1)
    ));
}