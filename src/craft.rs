use bevy::asset::AssetServer;
use bevy::hierarchy::{BuildChildren, Parent};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{default, Bundle, Commands, Component, Res, SpatialBundle, SpriteBundle, Transform};
use crate::tile::TileBundle;

#[derive(Component)]
pub struct Vessel{
    transform: Transform
}

impl Vessel{
    pub fn world_to_vessel_coords(&self, point: Vec2) -> Vec2{
        Vec2 {
            x: ((point.x - self.transform.translation.x) / 64f32).floor(),
            y: ((point.y - self.transform.translation.y) / 64f32).floor(),
        }
    }
}

pub fn spawn_vessel(mut commands: Commands, asset_server: Res<AssetServer>){
    
    
    commands.spawn((Vessel{
        transform: Transform{
            translation: Vec3::new(0f32, 0f32, 0f32),
            ..default()
        },
    }, SpatialBundle{
        ..default()
    })).with_children(|parent|{
    
        for i in 0..100{
            for j in 0..100{
                let tile = TileBundle
                {
                    sprite_bundle: SpriteBundle{
                        transform: Transform {
                            translation: Vec3::new(((i as f32) + 0.5f32) * 64 as f32, ((j as f32) + 0.5f32) * 64 as f32, 0f32),
                            ..default()
                        },
                        texture: asset_server.load("tiles/test.png"),
                        ..default()
                    }
                };
                parent.spawn(tile);
            }
        }
    });
    
}