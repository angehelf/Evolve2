use core::f32;


use std::f32::consts::PI;
use std::time::Instant;
use crate::collision_detection::*;
use crate::physics::*;
use crate::components::*;
use bevy::asset;
use bevy::color::palettes::css::BLUE;
use bevy::color::palettes::css::RED;
use bevy::log::tracing_subscriber::fmt::time;
use bevy::math::EulerRot::XYZ;
use bevy::{ecs::{query::QueryParManyIter, schedule::SingleThreadedExecutor, system::Single}, prelude::*, window::PrimaryWindow};
use rand::{RngExt, random_range};
use small_net_lib::*;
use crate::systems::*;
use rand_distr::{Normal,Distribution};
pub struct CreaturePlugin;

const DEG_TO_RAD : f32 = ((2.0* std::f32::consts::PI)/360.0);
const MAX_FORCE : f32 = 500.0;
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
   cooldown:f32,
   range:f32,
   fow:f32,
   number_of_ray:usize,
   closest_object_pos:Vec<f32>
}


 impl Creature{


pub fn spawn_random(commands: &mut Commands,asset_server : &Res<AssetServer>,window: &Single<&Window,With<PrimaryWindow>>){
    let mut rng = rand::rng();
    let window_size = window.resolution.physical_size();
    let random_size = rng.random_range(5.0..25.0);
    let random_heading:f32 =rng.random_range(0.0..360.0);
    let normal = Normal::new(0.0,1.0);
    let random_pos = vec2(rng.random_range(-(1800.0)..(1800.0)),rng.random_range(-(1800.0)..(1800.0)));
   
    let mut brain = SmallNet::new_grid(vec![2,16,16,2]);
    brain.initialize_activation_functions(ActivationInitType::PerLayer, vec![relu,relu,tanh]);
    brain.initialize_connections(ConnectionInitType::FullyConnected, || normal.unwrap().sample(&mut rng));
    brain.initialize_bias(|| 0.01);

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
    RigidBody{mass:random_size,force:vec2(5.0, 0.0),restitution_coef:0.25,..Default::default()

    },
   
    
    Healt(random_size*10.0),
    FoodReserve(random_size*10.0),
    EnergyReserve(random_size*10.0),
    Brain{
        brain 
    },
    
    Collider{shape:ColliderShape::Circle { radius: random_size/2.0 }}, 
    CollisionList::default(),
    Vision{timer:0.0,cooldown:0.1,range:200.0,fow:90.0,number_of_ray:5,closest_object_pos: vec![0.0,0.0]}
    
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

pub fn think(mut query: Query<(&mut Transform,&mut Brain,&mut RigidBody,&Vision)>,mut gizmos : Gizmos){
    
    query.par_iter_mut().for_each(|(mut transform, mut brain,mut rigidBody,vision)|{

        //let inputs = vec![transform.translation.x,transform.translation.y];
        let inputs = &vision.closest_object_pos;
        let outputs = brain.brain.feed_forward(inputs);
        transform.rotation =Quat::from_rotation_z( outputs[0]*PI);
        
         let dir =(transform.rotation*Vec3::Y).truncate();
        rigidBody.force += outputs[1].abs()*MAX_FORCE*dir;
        //gizmos.line_2d(transform.translation.truncate(), transform.translation.truncate()+outputs[1].abs()*MAX_FORCE*dir*0.1, BLUE);
        // println!("output: {:?}, {}",outputs[0]*PI/DEG_TO_RAD,outputs[1]);
        
    });

}

pub fn see(grid : Res<CollisionGrid>,target_query: Query<(&Transform,&Collider)>,mut entity_query: Query<(Entity,&Transform,&mut Vision,&mut Brain)>,time: Res<Time>,mut gizmos:Gizmos){
    let start = Instant::now();
    entity_query.par_iter_mut().for_each(|(entity,transform,mut vision,mut brain)|{
       
       if vision.timer < vision.cooldown {
        vision.timer += time.delta_secs();
        return;}
        
        let mut dir:Vec2;
        let mut ray:Ray2D ;
        let angle_between_ray = vision.fow/(vision.number_of_ray as f32-1.0);
        

        let mut closest_distance = f32::MAX;
        let mut closest_intercection_option: Option<RayCastIntersection> = None;
        let mut angle = 0.0;
        for _i in 0..vision.number_of_ray{

            let rotation =Quat::from_rotation_z( ((-vision.fow*0.5*DEG_TO_RAD)+_i as f32 *angle_between_ray*DEG_TO_RAD)+transform.rotation.to_euler(XYZ).2);
            dir=(rotation*Vec3::Y).truncate();
            ray = Ray2D { origin: transform.translation.truncate(), direction:dir , range: vision.range};
           
           // gizmos.line_2d(transform.translation.truncate(), transform.translation.truncate()+dir*vision.range, RED);
           if let Some(cast_result) =cast_rays(entity, &ray, &grid, &target_query).closest(){

            if cast_result.distance>= closest_distance{continue;}
            closest_distance=cast_result.distance;
            closest_intercection_option = Some(cast_result);
            angle = rotation.to_euler(XYZ).2;
           }
        }
        
        if closest_intercection_option.is_none() {vision.closest_object_pos=vec![0.0,0.0]; return;}
        let target_position=vec![closest_distance/vision.range,angle/PI];
         //println!("ray parameter: {:?}",target_position);
        vision.closest_object_pos=target_position;
      
        vision.timer =0.0;
        
    });
    
}