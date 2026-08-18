use std::f32::consts::PI;

use bevy::asset::transformer::TransformedSubAsset;
use bevy::prelude::*;
use crate::collision_detection::Ray2D;
use crate::components::*;
use crate::resources::*;

///Met à jour la position des entités avec la valeur de leur composant Speed
pub fn move_with_speed(time: Res<Time>, mut query:Query<(&mut Transform,&Speed)>){

for (mut transform,speed)in query.iter_mut(){

    transform.translation += speed.0.extend(0.0)*time.delta_secs();


}



}
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


pub fn speed_vector_from_heading(speed:f32,heading:f32)-> Speed{
    let heading_radian = heading* ((2.0*PI)/360.0);
let x = f32::cos(heading_radian)*speed;
let y = f32::sin(heading_radian)*speed;
Speed(
vec2(x, y)

)

}
pub fn turn_from_heading_speed(query:Query<(&mut Transform,&mut Heading, &HeadingSpeed,&mut Speed)>,time:Res<Time>){

for (mut transform,mut heading,rot_speed,mut speed) in query{
    let rot_speed_radian = rot_speed.0* ((2.0*PI)/360.0);

    heading.0+=rot_speed.0*time.delta_secs();

    transform.rotate_z(rot_speed_radian*time.delta_secs());
    
    let magnetude = speed.0.length();

    let heading_radian = heading.0* ((2.0*PI)/360.0);

    speed.0= vec2(magnetude * -f32::sin(heading_radian),magnetude * f32::cos(heading_radian));
    
  
}
   
}



