mod creature;
mod components;
mod systems;
mod camera;
mod resources;
mod plant;
mod collision_detection;
use core::f32;


use bevy::{prelude::*, window::{PrimaryWindow, WindowResolution}};
use crate::{collision_detection::{ColliderShape, CollisionDetectionPLugin, CollisionGrid, Ray2D, check_in_cell_or_overlap}, components::Size, creature::{Creature, CreaturePlugin}, resources::{DebugSettings, MapBorder, MapBorderType}};
use rand::{RngExt, random_range};
const DEG_TO_RAD : f32 = (2.0* std::f32::consts::PI)/360.0;




fn main() {
    App::new()
    .insert_resource(DebugSettings{
        show_rays:true,
        show_grid:true,
        show_collider:true,
    })
    .add_plugins(DefaultPlugins.set(WindowPlugin{
        primary_window: Some(Window{
            resolution: WindowResolution::new(1920,960).with_scale_factor_override(1.0),
            position:WindowPosition::Centered(MonitorSelection::Primary),
            resizable: false,
            ..default()
        }),
        ..default()
    }))
    .add_plugins(camera::CameraPlugin)
    
    .insert_resource(MapBorder{
        map_border_type:MapBorderType::Loop,
        map_size:vec2(2000.0,2000.0 )
    })
    .add_systems(Startup, setup)
    
    .add_plugins(CreaturePlugin)
    .add_systems(FixedUpdate, (systems::move_with_speed,systems::turn_from_heading_speed,systems::correct_transform_from_boundaries.after(systems::turn_from_heading_speed).after(systems::move_with_speed)))
    .add_systems(FixedUpdate, debug)
    .add_plugins(CollisionDetectionPLugin)
    .run();

}





fn setup(mut commands: Commands,asset_server:Res<AssetServer>,window:Single<&Window,With<PrimaryWindow>>,mut gizmos: Gizmos,t:ResMut<Time<Virtual>>)
{
    
    
    let mut rng = rand::rng();
    for _i in 0..1000{
        creature::Creature::spawn_random(&mut commands, &asset_server, &window);
        
        
    }
    for _i in 0..1000{
        
        let random_translation = vec3(rng.random_range(-500.0..500.0), rng.random_range(-500.0..500.0), 0.0);
        let random_rotation = vec3(0.0,0.0,DEG_TO_RAD*random_range(0.0..360.0));
        
        plant::Plant::spawn_new_plant(&mut commands,random_translation,random_rotation.z,10.0,&asset_server);
        
    }
    
    

   
 
}




fn debug(query : Query<(&Transform,&Size)>,mut gizmos : Gizmos){

    for (transform,size) in query{

        //gizmos.circle_2d(Isometry2d::from_translation(transform.translation.truncate()), size.0/2.0, Color::srgb(0.0, 255.0, 0.0));
    }

}



