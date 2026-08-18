use core::f32;



use crate::collision_detection::*;

use crate::components::*;
use bevy::asset;
use bevy::{ecs::{query::QueryParManyIter, schedule::SingleThreadedExecutor, system::Single}, prelude::*, window::PrimaryWindow};
use rand::{RngExt, random_range};
use small_net_lib::*;
use crate::systems::*;
use rand_distr::{Normal,Distribution};
pub struct CreaturePlugin;

const DEG_TO_RAD : f32 = ((2.0* std::f32::consts::PI)/360.0);

impl Plugin for CreaturePlugin{

fn build(&self, app: &mut App){

app.add_systems(FixedUpdate, update_energy);
app.add_systems(FixedUpdate, think);

}

}



#[derive(Component)]
pub struct Creature;

#[derive(Component)]
pub struct FoodReserve(f32);

#[derive(Component)]
pub struct  EnergyReserve(f32);

#[derive(Component)]
pub struct Brain{
    brain : SmallNet
}

 impl Creature{


pub fn spawn_random(commands: &mut Commands,asset_server : &Res<AssetServer>,window: &Single<&Window,With<PrimaryWindow>>){
    let mut rng = rand::rng();
    let window_size = window.resolution.physical_size();
    let random_size = rng.random_range(5.0..25.0);
    let random_heading:f32 =rng.random_range(0.0..360.0);
    let normal = Normal::new(0.0,0.01);
  
    let mut brain = SmallNet::new_grid(vec![2,6,1]);
    brain.initialize_activation_functions(ActivationInitType::PerLayer, vec![relu,tanh]);
    brain.initialize_connections(ConnectionInitType::FullyConnected, || normal.unwrap().sample(&mut rng));
    brain.initialize_bias(|| normal.unwrap().sample(&mut rng));

    let parent= commands.spawn((
    Creature,
    Sprite{
        
        custom_size: Some(Vec2::splat(random_size)),
        ..Sprite::from_image(asset_server.load("sprites/Creature_1_0.png"))
    },
    Transform{
        translation: vec3(rng.random_range(-(window_size.x as f32 *0.5)..(window_size.x as f32 *0.5)),
    rng.random_range(-(window_size.y as f32 *0.5)..(window_size.y as f32 *0.5)),0.0),
        rotation: Quat::from_rotation_z(random_heading*DEG_TO_RAD),
        ..Default::default()
    },
    Size(random_size),
    speed_vector_from_heading(30.0, random_heading),
   
    Heading(random_heading),
    HeadingSpeed(0.0),
    Healt(random_size*10.0),
    FoodReserve(random_size*10.0),
    EnergyReserve(random_size*10.0),
    Brain{
        brain 
    },
    GameObject{enable_collision:true}
   
    )).id();
   
   commands.spawn(Ray2D::from_parent(Transform::default(), 100.0, parent));
 
}
   


}



fn update_energy(mut commands:Commands, query: Query<(&Speed,&mut EnergyReserve,&Size,Entity)>,time: Res<Time>){


for (speed,mut energy_level,size,entity) in query{
    
let delta = size.0*0.00025 + size.0*speed.0.length()*time.delta_secs()*0.0005;
energy_level.0 -= delta;

if energy_level.0<= 0.0{

    commands.entity(entity).despawn();
}


}
}

pub fn think(query: Query<(&Transform,&mut Brain,&mut HeadingSpeed)>){
    
    for (&transform, mut brain,mut heading_speed) in query{

        let inputs = vec![transform.translation.x,transform.translation.y];
        heading_speed.0= brain.brain.feed_forward(&inputs)[0]*100.0;

        
    }

}