use bevy::prelude::*;

// GAMESTATE COMPONENTS

#[derive(States, Hash, Clone, PartialEq, Eq, Debug, Default)]
pub enum GameState { #[default] Loading, Ready }

//TEXTURE ATLAS COMPONENTS

#[derive(Resource, Default)]
pub struct ImageAssets {
    pub image: Handle<Image>,             // the `image` field here is only used to query the load state, lots of the
    pub tileset: Handle<TextureAtlas>,    // code in this file disappears if something like bevy_asset_loader is used.
}

//ANIMATION COMPONENTS

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
//Declaring how much images animation will use
pub struct Animation{
    pub frames: Vec<usize>,
    pub counter: usize,
    pub timer: Timer,
}

//CAMERA COMPONENTS

#[derive(Component)]
pub struct FaceCamera{}

//ENTITIES COMPONENTS
//TODO: Think about creating entities file wich would store all entity components

#[derive(Component)]
pub struct Player{}


