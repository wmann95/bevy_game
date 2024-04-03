use bevy::{ecs::{bundle::Bundle, component::Component}, sprite::{ColorMaterial, MaterialMesh2dBundle}};



#[derive(Component, Clone, Default)]
pub enum TileType{
    Object,
    #[default]
    Floor,
    Wall
}

#[derive(Component, Clone, Default)]
pub struct Tile;

#[derive(Bundle, Clone, Default)]
pub struct TileBundle{
    pub tile: Tile,
    pub tile_type: TileType,
    pub tile_texture: MaterialMesh2dBundle<ColorMaterial>
}

// MaterialMesh2dBundle{
//     mesh: meshes.add(Rectangle::default()).into(),
//     material: materials.add(ColorMaterial{
//         color: Color::GRAY,
//         texture: Some(asset_server.load("tiles/test.png")),
//     }),
//     transform: Transform {
//         translation: Vec3::new(((i as f32) + 5f32) * 32 as f32, ((j as f32) + 0.5f32) * 32 as f32, 0f32),
//         scale: Vec3::splat(32.),
//         ..default()
//     },
//     ..default()
// },