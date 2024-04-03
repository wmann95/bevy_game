use core::hash;
use std::collections::HashMap;
use bevy::asset::{AssetServer, Assets};
use bevy::ecs::bundle::Bundle;
use bevy::ecs::system::{EntityCommands, ResMut};
use bevy::hierarchy::BuildChildren;
use bevy::math::primitives::Rectangle;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{default, Commands, Component, Res};
use bevy::render::color::Color;
use bevy::render::mesh::Mesh;
use bevy::render::view::{InheritedVisibility, ViewVisibility, Visibility};
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle};
use bevy::transform::components::{GlobalTransform, Transform};

use super::tile::{Tile, TileBundle, TileType};
use super::tilemap::{TileMap, TileStack};

#[derive(Bundle, Default)]
pub struct VesselBundle{
    vessel: Vessel,
    tile_map: TileMap,
    vessel_position: VesselPosition,
    vessel_velocity: VesselVelocity,
    vessel_acceleration: VesselAcceleration,
    visibility: Visibility,
    inherited_visibility: InheritedVisibility,
    view_visibility: ViewVisibility,
    transform: Transform,
    global_transform: GlobalTransform
}

#[derive(Component, Default)]
pub struct Vessel;

#[derive(Component, Default)]
pub struct VesselPosition(pub Vec2);

#[derive(Component, Default)]
pub struct VesselVelocity(pub Vec2);
#[derive(Component, Default)]
pub struct VesselAcceleration(pub Vec2);

impl VesselBundle{
    pub fn new(position: Vec2, velocity: Vec2, acceleration: Vec2) -> Self{
        Self{
            vessel: Vessel,
            tile_map: TileMap::default(),
            vessel_position: VesselPosition(position),
            vessel_velocity: VesselVelocity(velocity),
            vessel_acceleration: VesselAcceleration(acceleration),
            visibility: Visibility::Visible,
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
            global_transform: GlobalTransform::default()
        }
    }
}

pub fn spawn_dev_vessel(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
){
    
    let craft = commands.spawn(VesselBundle{
        vessel: Vessel,
        tile_map: TileMap::new(),
        ..default()
    });
    
    for i in 0..30{
        
        for j in 0..30{
            if i == 0 || i == 29 || j == 0 || j == 29{
                TileStack(vec![TileBundle{
                    tile_type: TileType::Wall,
                    tile_texture: MaterialMesh2dBundle{
                        mesh: meshes.add(Rectangle::default()).into(),
                        material: materials.add(ColorMaterial{
                            color: Color::GRAY,
                            texture: Some(asset_server.load("tiles/test.png")),
                        }),
                        // do this with a system.
                        // transform: Transform {
                        //     translation: Vec3::new(((i as f32) + 5f32) * 32 as f32, ((j as f32) + 0.5f32) * 32 as f32, 0f32),
                        //     scale: Vec3::splat(32.),
                        //     ..default()
                        // },
                        ..default()
                    },
                    ..default()
                }; 1]);
            }
            else{
                let stack = TileStack(vec![TileBundle{
                    tile_texture: MaterialMesh2dBundle{
                        mesh: meshes.add(Rectangle::default()).into(),
                        material: materials.add(ColorMaterial{
                            color: Color::WHITE,
                            texture: Some(asset_server.load("tiles/test.png")),
                        }),
                        // transform: Transform {
                        //     translation: Vec3::new(((i as f32) + 5f32) * 32 as f32, ((j as f32) + 0.5f32) * 32 as f32, 0f32),
                        //     scale: Vec3::splat(32.),
                        //     ..default()
                        // },
                        ..default()
                    },
                    ..default()
                }; 1]);
                commands.spawn(stack);
            }
        }
    }
    
    
    
    
    // craft.add_child()
    
    // commands.entity(c)
    
}

// pub fn spawn_dev_vessel(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     asset_server: Res<AssetServer>,
//     mut materials: ResMut<Assets<ColorMaterial>>
// ){
//     let vessel_transforms = HashMap::new();
    
//     let vessel = commands.spawn(
//         Vessel{
//             vessel_transforms
//         }
//     ).id();
    
//     commands.entity(vessel).insert(
//         SpatialBundle{
//             transform: Transform{
//                 translation: Vec3::new(0f32, 0f32, 0f32),
//                 ..default()
//                 },
//             ..default()
//         }
//     );
    
//     for i in 0..30{
//         for j in 0..30{
//             if i == 0 || i == 29 || j == 0 || j == 29{
//                 commands.spawn((
//                     MaterialMesh2dBundle{
//                         mesh: meshes.add(Rectangle::default()).into(),
//                         material: materials.add(ColorMaterial{
//                             color: Color::GRAY,
//                             texture: Some(asset_server.load("tiles/test.png")),
//                         }),
//                         transform: Transform {
//                             translation: Vec3::new(((i as f32) + 5f32) * 32 as f32, ((j as f32) + 0.5f32) * 32 as f32, 0f32),
//                             scale: Vec3::splat(32.),
//                             ..default()
//                         },
//                         ..default()
//                     },
//                     Tile{
//                         vessel_transform: VesselTransform{
//                             x_position: j,
//                             y_position: i,
//                             tiles: Vec::new(),
//                         },
//                         tile_type: TileType::Wall
//                     }
//                 ));
//             }
//             else{
//                 commands.spawn((
//                     MaterialMesh2dBundle{
//                         mesh: meshes.add(Rectangle::default()).into(),
//                         material: materials.add(ColorMaterial{
//                             color: Color::WHITE,
//                             texture: Some(asset_server.load("tiles/test.png")),
//                         }),
//                         transform: Transform {
//                             translation: Vec3::new(((i as f32) + 5f32) * 32 as f32, ((j as f32) + 0.5f32) * 32 as f32, 0f32),
//                             scale: Vec3::splat(32.),
//                             ..default()
//                         },
//                         ..default()
//                     },
//                     Tile{
//                         vessel_transform: VesselTransform{
//                             x_position: j,
//                             y_position: i,
//                             tiles: Vec::new(),
//                         },
//                         tile_type: TileType::Floor
//                     },
//                     Atmosphere{
//                         o2: 209460.,
//                         n2: 105420.,
//                         co2: 0.,
//                         temperature: 21.
//                     }
//                 ));
//             }
//         }
//     }
// }

// TODO: Implement flood-fill here: https://en.wikipedia.org/wiki/Flood_fill
// pub fn setup_vessel_rooms(
//     mut commands: Commands,
//     vessels: Query<(&Vessel, &Children)>,
//     tiles: Query<(&Transform, &Atmosphere), With<Tile>>
// ){
//     for (vessel, children) in vessels.iter(){
//         let mut stack = Vec::new();
//         let some_tile = *children.iter().next().expect("Vessel can't have no tiles!");
        
//         stack.push(some_tile);
        
//         while !stack.is_empty(){
//             let entity = stack.pop().expect("Stack cannot be empty here.");
//             let (tile_transform, tile_atmosphere) = tiles.get(entity).expect("Tile cannot be missing.");
            
//         }
//     }
// }