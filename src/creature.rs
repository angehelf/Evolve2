use core::f32;


use std::time::Instant;
use crate::collision_detection::*;

use crate::components::*;
use bevy::asset;
use bevy::log::tracing_subscriber::fmt::time;
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
app.add_systems(FixedUpdate, see);
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
#[derive(Component)]
pub struct Vision{
   timer :f32,
   cooldown:f32

}


 impl Creature{


pub fn spawn_random(commands: &mut Commands,asset_server : &Res<AssetServer>,window: &Single<&Window,With<PrimaryWindow>>){
    let mut rng = rand::rng();
    let window_size = window.resolution.physical_size();
    let random_size = rng.random_range(5.0..25.0);
    let random_heading:f32 =rng.random_range(0.0..360.0);
    let normal = Normal::new(0.0,0.01);
    let random_pos = vec2(rng.random_range(-(1800.0)..(1800.0)),rng.random_range(-(1080.0)..(1800.0)));
   
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
        translation: random_pos.extend(0.0),
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
    GameObject{enable_collision:true},
    Collider{shape:ColliderShape::Circle { radius: random_size/2.0 }},  
    Vision{timer:0.0,cooldown:0.1}
    
    ));
   
    
      
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

pub fn think(mut query: Query<(&Transform,&mut Brain,&mut HeadingSpeed)>){
    
    query.par_iter_mut().for_each(|(&transform, mut brain,mut heading_speed)|{

        let inputs = vec![transform.translation.x,transform.translation.y];
        heading_speed.0= brain.brain.feed_forward(&inputs)[0]*100.0;

        
    });

}

pub fn see(grid : Res<CollisionGrid>,target_query: Query<(&Transform,&Collider)>,mut entity_query: Query<(Entity,&Transform,&Heading,&mut Vision)>,time: Res<Time>){
    let start = Instant::now();
    entity_query.par_iter_mut().for_each(|(entity,transform,heading,mut vision)|{
       
       if vision.timer < vision.cooldown {
        vision.timer += time.delta_secs();
        return;}
        
        let dir =(transform.rotation*Vec3::Y).truncate();
         
        let ray = Ray2D { origin: transform.translation.truncate(), direction:dir , range: 200.0 };
        //gizmos.line_2d(ray.origin, ray.get_end_point(), Color::srgb(255.0, 0.0, 0.0));
        cast_rays(entity, &ray, &grid, &target_query);
        vision.timer =0.0;
        
    });
    
}