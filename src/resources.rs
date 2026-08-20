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

#[derive(Resource)]
pub struct DebugSettings{

pub show_rays: bool,
pub show_grid:bool,
pub show_collider: bool

}