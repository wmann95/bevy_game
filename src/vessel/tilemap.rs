use std::collections::HashMap;

use bevy::ecs::component::Component;

use super::tile::TileBundle;

#[derive(Component, Clone)]
pub struct TileStack(pub Vec<TileBundle>);

#[derive(Component, Default)]
pub struct TileMap(HashMap<i32, HashMap<i32, TileStack>>);

impl TileMap{
    
    pub fn new() -> Self {
        Self(HashMap::<i32, HashMap<i32, TileStack>>::new())
    }
    
    pub fn get_tile_stack(&self, x: i32, y: i32) -> Option<&TileStack>{
        let row = match self.0.get(&y){
            Some(r) => r,
            None => return None
        };
        
        match row.get(&x) {
            Some(c) => Some(c),
            None => None
        }
    }
    
    pub fn from_vec_vec(input: Vec<Vec<TileStack>>) -> Self{
        let mut hash_map = HashMap::new();
        
        input.iter().enumerate().for_each(|(j, row)| {
            let mut buffer_hashmap = HashMap::new();
            
            row.iter().enumerate().for_each(|(i, transform)|{
                buffer_hashmap.insert(i as i32, transform.to_owned());
            });
            
            hash_map.insert(j as i32, buffer_hashmap);
        });
        
        Self(hash_map)
    }
}
