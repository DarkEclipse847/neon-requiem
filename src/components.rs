use bevy::prelude::*;

#[derive(States, Hash, Clone, PartialEq, Eq, Debug, Default)]
pub enum GameState { #[default] Loading, Ready }

#[derive(Component)]
//Declaring how much images animation will use
pub struct Animation{
    pub frames: Vec<usize>,
    pub counter: usize,
    pub timer: Timer,
}
#[derive(Resource, Default)]
pub struct ImageAssets {
    pub image: Handle<Image>,        // the `image` field here is only used to query the load state, lots of the
    pub tileset: Handle<TextureAtlas>, // code in this file disappears if something like bevy_asset_loader is used.
}

#[derive(Component)]
pub struct Player{}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

