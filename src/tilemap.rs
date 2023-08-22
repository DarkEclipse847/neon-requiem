use bevy::{prelude::*, transform::commands};
use bevy_ecs_tilemap::prelude::*;

const QUADRANT_SIDE_LENGTH: u32 = 8;

pub struct CreateMapPlugin;

impl Plugin for CreateMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_map);
    }
}

fn create_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    let texture_handle: Handle<Image> = asset_server.load("sprites/tiles/tile.png");

    let map_size = TilemapSize{
        x: QUADRANT_SIDE_LENGTH*2,
        y: QUADRANT_SIDE_LENGTH*2,
    };
    let quadrant_size = TilemapSize{
        x: QUADRANT_SIDE_LENGTH,
        y: QUADRANT_SIDE_LENGTH,
    };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);
    fill_tilemap_rect(
        TileTextureIndex(0),
        TilePos { x: 0, y: 0 },
        quadrant_size,
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Isometric(IsoCoordSystem::Staggered);

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        map_type,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}




/*use bevy::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

impl Plugin for TilemapPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, create_map);
    }
}

//#[derive(Component)]
//pub struct Tile;

#[derive(Component)]
pub struct TileCollider;

pub struct TilemapPlugin;

#[derive(Resource)]
pub struct SpriteSheet{pub tiles: Handle<TextureAtlas>}


pub fn parse_sprite(
    mut commands: &Commands,
    sprite: &SpriteSheet,
    index: usize,
    translation: Vec3,
) -> Entity{
    let mut sprite_handler = TextureAtlasSprite::new(index);
    commands.spawn(SpriteSheetBundle{
        sprite: sprite_handler,
        texture_atlas: sprite.tiles.clone(),
        transform: Transform {
            translation: translation,
            ..Default::default()
        },
        ..default()
    }).id()
}

fn create_map(
    sprite: &SpriteSheet,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    let file = File::open("assets/maps/map.txt").expect("No tilemap found");
    let mut tiles: Vec<Entity> = Vec::new();

    for(y, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line) = line {
            for(x, char) in line.chars().enumerate(){
                let tile = parse_sprite(
                    &mut commands,
                    sprite,
                    char as usize,
                    Vec3::new(x as f32* 7.0, y as f32*7.0, 100.0),
                );
            }
        }
    }
}*/
