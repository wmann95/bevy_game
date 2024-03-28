mod craft;
mod tile;

use bevy::{prelude::*, window::{PresentMode, PrimaryWindow}};
use craft::Vessel;
use crate::craft::spawn_vessel;

const CAMERA_SPEED: f32 = 200.;


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
        .add_systems(Startup, spawn_vessel)
        .add_systems(Update, (mouse_click_system, keyboard_input_system))
        .run();
}

fn mouse_click_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>, 
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    vessel_query: Query<&Vessel>
){
    let window = windows.single_mut();
    let (camera, camera_transform) = camera_query.single();
    let vessel = vessel_query.single();
    
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else{
        return;
    };
    
    if mouse_button_input.just_pressed(MouseButton::Left){
        println!("{:?}", vessel.world_to_vessel_coords(point));
    }
}

fn world_to_tile_coords(point: Vec2) -> Vec2{
    Vec2 {
        x: (point.x / 64f32).floor(),
        y: (point.y / 64f32).floor(),
    }
}

fn keyboard_input_system(key_button_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Transform, With<Camera>>, time: Res<Time>){
    let mut camera_transform = query.single_mut();
    
    if key_button_input.pressed(KeyCode::KeyW){
        camera_transform.translation = Vec3::new(camera_transform.translation.x, camera_transform.translation.y + CAMERA_SPEED * time.delta_seconds(), camera_transform.translation.z);
    }
    if key_button_input.pressed(KeyCode::KeyS){
        camera_transform.translation = Vec3::new(camera_transform.translation.x, camera_transform.translation.y - CAMERA_SPEED * time.delta_seconds(), camera_transform.translation.z);
    }
    if key_button_input.pressed(KeyCode::KeyA){
        camera_transform.translation = Vec3::new(camera_transform.translation.x - CAMERA_SPEED * time.delta_seconds(), camera_transform.translation.y, camera_transform.translation.z);
    }
    if key_button_input.pressed(KeyCode::KeyD){
        camera_transform.translation = Vec3::new(camera_transform.translation.x + CAMERA_SPEED * time.delta_seconds(), camera_transform.translation.y, camera_transform.translation.z);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(Camera2dBundle::default());
    
    
    
}