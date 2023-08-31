use bevy::prelude::*;
use crate::components::*;
use bevy_sprite3d::*;


pub fn spawn_map(
    mut commands: Commands,
    assets: Res<ImageAssets>,
    //mut sprite_params: Sprite3dParams,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_xyz(-0.9, 0.5, -3.1),
        ..default()
    });
    let f = [(9,4)];
    // Need to add different textures to walls so it will less boring
    //let mut map = vec![
    //    !vec[f(), f(),f(),f(),f(),f(),f()],
    //    !vec[f(),f(),f(),f(),f(),f(),f()],
    //    !vec[f(),f(),f(),f(),f(),f(),f()],
    //    !vec[f(),f(),f(),f(),f(),f(),f()],
    //    !vec[f(),f(),f(),f(),f(),f(),f()],
    //    !vec[f(),f(),f(),f(),f(),f(),f()],
    //];

    // Creates a padding
    //map.insert(0, vec![(0,0); map[0].len()]);
    //map.push(vec![(0,0); map[0].len()]);
    //for row in map.iter_mut() {
    //    row.insert(0, (0,0));
    //    row.push((0,0));
    //}
}