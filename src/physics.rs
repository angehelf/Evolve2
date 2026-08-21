use bevy::{color::palettes::{css::*, tailwind::RED_100}, ecs::{entity_disabling, query::QueryItem}, math::VectorSpace, prelude::*};
use crate::{collision_detection::CollisionList, components::*};
#[derive(Component)]
pub struct RigidBody{

    pub mass: f32,
    pub force: Vec2,
    pub speed: Vec2,
    pub restitution_coef:f32
}
#[derive(Resource)]
pub struct PhysicsSetings{
    drag_coeficient:f32

}

impl Default for RigidBody{

    fn default() -> Self {
        Self{
        mass:0.0,
        force : Vec2::ZERO,
        speed: Vec2::ZERO,
        restitution_coef:1.0
        }
    }
}
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin{

fn build(&self, app: &mut App){
app.insert_resource(PhysicsSetings{
    drag_coeficient : 0.05
});
app.add_systems(FixedUpdate, aplly_linear_physics);
app.add_systems(FixedUpdate, drag);
app.add_systems(FixedUpdate, aplly_collision);

}

}



pub fn aplly_linear_physics(query:Query<(&mut Transform,&mut RigidBody)>,time:Res<Time>,mut gizmos:Gizmos){

    let delta_time_sec = time.delta_secs();
    for(mut transform,mut rigidbody) in query{

       let acceleration = rigidbody.force/rigidbody.mass;
      //  gizmos.line_2d(transform.translation.truncate(), transform.translation.truncate()+rigidbody.force*0.1, MAGENTA);
        rigidbody.speed +=acceleration*delta_time_sec;
      //   gizmos.line_2d(transform.translation.truncate(), transform.translation.truncate()+rigidbody.speed, GREEN);
        transform.translation += (rigidbody.speed*delta_time_sec).extend(0.0);

        rigidbody.force=Vec2::ZERO;
    }


}

pub fn drag(query:Query<(&mut RigidBody,&Transform)>,settings : Res<PhysicsSetings>,mut gizmos:Gizmos){

    for (mut rigid_body,transform) in query{

        let drag_force = -(rigid_body.speed*rigid_body.speed.length())*settings.drag_coeficient;
        rigid_body.force += drag_force;
       // gizmos.line_2d(transform.translation.truncate(), transform.translation.truncate()+drag_force*0.1, YELLOW);
    }

}

pub fn aplly_collision(mut query: Query<(&mut RigidBody,&CollisionList,Entity)>){
    let mut exit_speeds: Vec<(Vec2,Entity,Vec2,Entity)> = Vec::default();
    for (rigidbody, collision_list,entity) in &query{
      
        for collision in &collision_list.collisions{
            if entity<collision.with{continue;}
            
            let (target_rigidbody,_,_) = query.get(collision.with).unwrap();
            
            let relative_speed_along_normal = (rigidbody.speed-target_rigidbody.speed).dot(collision.normal);
            
            if relative_speed_along_normal <0.0 {continue;}

            let global_restitution_coef = rigidbody.restitution_coef*target_rigidbody.restitution_coef;
            
            let impulsion_factor= -(1.0+global_restitution_coef)*relative_speed_along_normal;
            let impulsion_denominator = (1.0/rigidbody.mass)+(1.0/target_rigidbody.mass);

            let impulse = impulsion_factor/impulsion_denominator;
           
            let exit_speed_vector= rigidbody.speed+(impulse/rigidbody.mass)*collision.normal;
            let target_exit_speed_vector = target_rigidbody.speed-(impulse/target_rigidbody.mass)*collision.normal;

            exit_speeds.push((exit_speed_vector,entity,target_exit_speed_vector,collision.with));
        }
    }

    for (exit_speed,this_entity,target_exit_speed,target_entity) in exit_speeds{
        query.get_mut(this_entity).unwrap().0.speed=exit_speed;
        query.get_mut(target_entity).unwrap().0.speed=target_exit_speed;
    }
}

