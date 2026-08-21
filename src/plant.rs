use bevy::prelude::*;

use crate::components::*;
use crate::collision_detection::*;
use crate::physics::RigidBody;

#[derive(Component)]
pub struct Plant{


}


impl Plant{
pub fn spawn_new_plant( command : &mut Commands,translation:Vec3,rotation:f32,scale:f32,asset_server:&Res<AssetServer>,speed: Vec2){

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

Size(scale),
Collider{shape:ColliderShape::Circle { radius: scale/2.0 }},
CollisionList::default(),
RigidBody{mass:scale,speed,restitution_coef:0.5,..Default::default()}
));
}
}