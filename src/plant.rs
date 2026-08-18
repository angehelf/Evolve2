use bevy::prelude::*;

use crate::components::*;

#[derive(Component)]
pub struct Plant{


}


impl Plant{
pub fn spawn_new_plant( command : &mut Commands,translation:Vec3,rotation:f32,scale:Vec3,asset_server:&Res<AssetServer>){

command.spawn((
Plant{},
Sprite::from_image(asset_server.load("sprites/trefle.png")),
Transform{
    translation,
    rotation : Quat::from_rotation_z(rotation),
    scale
},
GameObject{
    enable_collision:true
}


));
}
}