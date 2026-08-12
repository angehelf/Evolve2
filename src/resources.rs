use bevy::prelude::*;

#[derive(Resource)]
pub struct MapBorder{

   pub map_size:Vec2,
   pub map_border_type:MapBorderType
   
}
    
pub enum MapBorderType {
None,
Loop,
Wall
}