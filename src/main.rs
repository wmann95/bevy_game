mod vessel;
mod player;
mod physics;

use bevy::{prelude::*, window::{PresentMode, PrimaryWindow}};
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_mod_raycast::immediate::{Raycast, RaycastSettings};
use vessel::{atmosphere::Atmosphere, craft::spawn_dev_vessel};

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
        .add_plugins(DefaultPickingPlugins)
        .add_systems(Startup, (setup, spawn_dev_vessel).chain())
        .add_systems(Update, (mouse_click_system, keyboard_input_system))
        //.add_systems(Update, vessel_atmosphere_system)
        .run();
}

fn mouse_click_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>, 
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    // tile_q: Query<Option<&Atmosphere>>,
    mut raycast: Raycast,
    // mut commands: Commands
){
    let window = windows.single_mut();
    let (camera, camera_transform) = camera_query.single();
    
    
    if mouse_button_input.just_pressed(MouseButton::Left){
        let Some(cursor_position) = window.cursor_position() else {
            return;
        };
        
        let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else{
            return;
        };
        
        let ray = Ray3d::new(Vec3::new(point.x, point.y, 1.0), -Vec3::Z);
        let hits = raycast.cast_ray(ray, &RaycastSettings::default());
        
        for (entity, intersection_data) in hits{
            println!("{} {}", entity.index(), intersection_data.position());
            // let atmosphere = tile_q.get(*entity).unwrap();
            // if atmosphere.is_none() { println!("Wall"); }
            // else {
            //     println!("{}", atmosphere.unwrap().o2);
            // };
            
        }
    }
    // if mouse_button_input.just_pressed(MouseButton::Left){
    //     println!("{:?}", vessel.world_to_vessel_coords(point));
    // }
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

fn setup(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
}