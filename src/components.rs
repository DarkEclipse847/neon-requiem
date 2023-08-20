use bevy::prelude::*;

#[derive(Component)]
//Declaring how much images animation will use
pub struct AnimationIndices{
    pub first: usize,
    pub last: usize,
}

#[derive(Component)]
pub struct Player{}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);