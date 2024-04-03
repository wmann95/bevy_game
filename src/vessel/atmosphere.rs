use bevy::{
    ecs::{component::Component, query::With, system::{Commands, Query, Res}}, hierarchy::Children, time::{Real, Time}, transform::components::Transform
};
use bevy_mod_raycast::immediate::Raycast;

use super::{craft::Vessel, tile::Tile};


pub const ATMOS_DRAG: f32 = 0.9;
pub const UPDATES_PER_SECOND: i32 = 5;

#[derive(Component)]
pub struct Atmosphere{
    pub o2: f32,
    pub n2: f32,
    pub co2: f32,
    pub temperature: f32
}

impl Atmosphere{
    pub fn get_pressure(&self) -> f32{
        self.o2 + self.n2 + self.co2
    }
}


