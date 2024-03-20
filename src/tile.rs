use bevy::prelude::{Bundle, Component, SpriteBundle};

#[derive(Component)]
struct Tile{
    
}

#[derive(Bundle)]
pub struct TileBundle{
    sprite_bundle: SpriteBundle
}