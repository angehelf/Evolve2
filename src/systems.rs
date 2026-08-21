use std::f32::consts::PI;

use bevy::asset::transformer::TransformedSubAsset;
use bevy::prelude::*;
use crate::collision_detection::Ray2D;
use crate::components::*;
use crate::resources::*;


///Corrige la position d'une entité en fonction des limite autorisées.
pub fn correct_transform_from_boundaries(mut transform_query: Query<&mut Transform,Without<Camera>> ,border_type:Res<MapBorder>){
   let map_size=border_type.map_size;
match border_type.map_border_type{
MapBorderType::None=>{

}
MapBorderType::Loop =>{
    
for mut transform in transform_query.iter_mut(){
    if transform.translation.x >= map_size.x{
        transform.translation.x = -map_size.x;

    }
    else if transform.translation.x <= -map_size.x{
        transform.translation.x = map_size.x;
    }
    if transform.translation.y >= map_size.y{
        transform.translation.y = -map_size.y;
    }
    else if transform.translation.y <= -map_size.y{
        transform.translation.y = map_size.y;
    }
}


}

MapBorderType::Wall=>{

    for mut transform in transform_query.iter_mut(){
    if transform.translation.x >= map_size.x{
        transform.translation.x = map_size.x;
    }
    else if transform.translation.x <= -map_size.x{
        transform.translation.x = -map_size.x;
    }
    if transform.translation.y >= map_size.y{
        transform.translation.y = map_size.y;
    }
   else if transform.translation.y <= -map_size.y{
        transform.translation.y = -map_size.y;
    }
}

}
    
}
}






