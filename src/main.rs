mod creature;
mod components;
mod systems;
mod camera;
mod resources;

use bevy::{prelude::*, window::{PrimaryWindow, WindowResolution}};
use crate::{creature::CreaturePlugin, resources::{MapBorder, MapBorderType}};


fn main() {
    App::new()
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
    .run();

}





fn setup(mut commands: Commands,asset_server:Res<AssetServer>,window:Single<&Window,With<PrimaryWindow>>)
{
   

    for _i in 0..1{
        creature::Creature::spawn_random(&mut commands, &asset_server, &window);
    }
    
 
}
