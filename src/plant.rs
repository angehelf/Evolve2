use bevy::prelude::*;

use crate::components::*;
use crate::collision_detection::*;

#[derive(Component)]
pub struct Plant{


}


impl Plant{
pub fn spawn_new_plant( command : &mut Commands,translation:Vec3,rotation:f32,scale:f32,asset_server:&Res<AssetServer>){

command.spawn((
Plant{},
Sprite{
     custom_size:Some(Vec2::splat(scale)),
     ..Sprite::from_image(asset_server.load("sprites/trefle.png"))

},
Transform{
    translation,
    rotation : Quat::from_rotation_z(rotation),
    ..Default::default()
},
GameObject{
    enable_collision:true
},
Size(scale),
Collider{shape:ColliderShape::Circle { radius: scale/2.0 }}
));
}
}