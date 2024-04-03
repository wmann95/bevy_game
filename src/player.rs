use bevy::{ecs::component::Component, math::Vec2};


#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Position(Vec2);

#[derive(Component)]
pub struct Velocity(Vec2);

#[derive(Component)]
pub struct Acceleration(Vec2);