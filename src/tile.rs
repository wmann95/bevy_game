use bevy::prelude::{Bundle, Component, SpriteBundle};



#[derive(Bundle)]
pub struct TileBundle{
    pub(crate) sprite_bundle: SpriteBundle,
}