mod craft;
mod tile;

use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, window::PresentMode};

fn main() {
    App::new().add_plugins(DefaultPlugins.set(WindowPlugin{
        primary_window: Some(Window {
            resolution: (800.,600.).into(),
            present_mode: PresentMode::AutoVsync,
            ..default()
        }),
        ..default()
    }))
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>){
    commands.spawn(Camera2dBundle::default());
    
    let shape = Mesh2dHandle(meshes.add(Circle{ radius: 50.0 }));
    
    let color = Color::rgb(1.0, 0.5, 0.5);
    
    commands.spawn(MaterialMesh2dBundle{
        mesh: shape,
        material: materials.add(color),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}