use std::cell;
use std::time::Instant;
use crate::{components::*, resources::DebugSettings};
use bevy::{
    ecs::entity::EntityHashSet, gizmos::gizmos::GizmoStorage, log::tracing_subscriber::fmt::time, math::EulerRot::XYZ, platform::collections::HashMap, prelude::*,
};
use num_traits::Pow;

const GRID_SIZE: u32 = 60;
const CELL_BORDER_PADDING:f32 = 0.01;

pub struct Ray2D {
    pub origin: Vec2,
    pub direction : Vec2,
    pub range: f32,
    //pub source: Entity
}

#[derive(Component)]
pub struct Collider {
   pub shape: ColliderShape
}
#[derive(Component,Default)]
pub struct RaycastCoolDown{
    pub clock:f32,
    pub cooldown: f32
}

pub enum ColliderShape {
    Circle{radius : f32},
    Rect{size: Vec2}
}

    


#[derive(Resource, Default)]
pub struct CollisionGrid {
    grid: HashMap<(i32, i32), EntityHashSet>,
    entity_cell : HashMap<Entity,Vec<(i32,i32)>>
}

pub struct CollisionDetectionPLugin;
impl Plugin for CollisionDetectionPLugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CollisionGrid>();
        app.add_systems(PostUpdate,  (draw_grid,draw_collider).run_if(|debug: Res<DebugSettings>| debug.show_collider));
        app.add_observer(add_to_grid);
        app.add_systems(PostUpdate, update_grid);
        
        
        
    }
}

fn draw_collider(mut gizmos: Gizmos, debug_settings: Res<DebugSettings>,query:Query<(&Transform,&Collider)>) {
 
    for (transform,collider) in query{
        match collider.shape {
            
            ColliderShape::Circle { radius }=>{
                gizmos.circle_2d(Isometry2d::from_translation(transform.translation.truncate()), radius, Color::srgb(0.0, 255.0, 0.0));
            }

            ColliderShape::Rect { size }=>{
                gizmos.rect_2d(Isometry2d::from_translation(transform.translation.truncate()), size, Color::srgb(0.0, 255.0, 0.0));
            }
        }
    }
}
fn draw_grid(mut gizmos: Gizmos, debug_settings: Res<DebugSettings>) {
    if !debug_settings.show_grid {
        return;
    }
    gizmos.circle_2d(
        Isometry2d::from_translation(vec2(0.0, 0.0)),
        5.0,
        Color::srgb(0.0, 0.0, 255.0),
    );
    gizmos.grid_2d(
        Isometry2d::from_translation(vec2(0.0, 0.0)),
        uvec2(100, 100),
        vec2(GRID_SIZE as f32, GRID_SIZE as f32),
        Color::srgba(60.0, 60.0, 60.0,0.5),
    );
}

pub fn add_to_grid(
    add: On<Add, GameObject>,
    query: Query<(&Transform,&Collider)>,
    mut colision_grid: ResMut<CollisionGrid>,
    mut gizmos : Gizmos
) {
    
    let (entity_transform,collider) = query.get(add.entity).unwrap();

    //let grid_coordinate: (i32, i32) = world_to_grid_coordinate(entity_transform.translation);
    let grid_coordinate_tab = check_in_cell_or_overlap(&entity_transform.translation.truncate(), &collider.shape);
    for grid_coordinate in &grid_coordinate_tab {
    colision_grid
        .grid
        .entry(*grid_coordinate)
        .or_default()
        .insert(add.entity);
    // println!("components position : {:?} ajouté a la grille index {:?}",entity_transform.translation.truncate(),grid_coordinate);
    }
    colision_grid.entity_cell.insert(add.entity, grid_coordinate_tab);
}

fn update_grid(query: Query<(Entity,&Transform,&Collider),With<GameObject>>,mut colision_grid: ResMut<CollisionGrid>, mut gizmos: Gizmos){
//let start = Instant::now();
        
    for (entity,transform,collider) in query{

        //let grid_coordinate: (i32, i32) = world_to_grid_coordinate(&transform.translation);
        let grid_coordinate_tab = check_in_cell_or_overlap(&transform.translation.truncate(), &collider.shape);
        let prev_index = colision_grid.entity_cell.get(&entity).unwrap().clone();
       
        //println!("previous : {:?}, actual : {:?}",prev_index,grid_coordinate);
        
        /* for h in colision_grid.entity_cell.get(&entity).unwrap(){
            
            gizmos.rect_2d(Isometry2d::from_translation(vec2((h.0 as f32 * GRID_SIZE as f32)+(GRID_SIZE as f32*0.5), (h.1 as f32* GRID_SIZE as f32)+(GRID_SIZE as f32*0.5))), Vec2::splat(GRID_SIZE as f32), Color::srgb(255.0, 0.0, 0.0));
            
        } */

      /*   for h in &colision_grid.grid{
            
                gizmos.rect_2d(Isometry2d::from_translation(vec2((h.0.0 as f32 * GRID_SIZE as f32)+(GRID_SIZE as f32*0.5), (h.0.1 as f32* GRID_SIZE as f32)+(GRID_SIZE as f32*0.5))), Vec2::splat(GRID_SIZE as f32), Color::srgb(255.0, 0.0, 255.0));
                 
        } */
        
        if grid_coordinate_tab == *prev_index {continue;}
        for prev in prev_index{
            
            if let Some(entity_collection)=colision_grid.grid.get_mut(&prev){
                entity_collection.remove(&entity);
                if entity_collection.is_empty(){
                    colision_grid.grid.remove(&prev);
                }
            }
            
        }
        
        for  grid_coordinate in & grid_coordinate_tab{
            
            colision_grid
            .grid
            .entry(*grid_coordinate)
            .or_default()
            .insert(entity);
            // println!("entity: {:?} juste changed from {:?} to {:?}", entity,prev_index,grid_coordinate);
            
        }
        colision_grid.entity_cell.insert(entity, grid_coordinate_tab);

    }
    // println!("passé : {:?}",start.elapsed().as_micros())
}





fn world_to_grid_coordinate(position: &Vec2) -> (i32, i32) {
    (
        (position.x / GRID_SIZE as f32).floor() as i32,
        (position.y / GRID_SIZE as f32).floor() as i32,
    )
}
fn grid_coordinate_to_world(position: &(i32, i32)) -> Vec2 {
    vec2(
        position.0 as f32 * GRID_SIZE as f32,
        position.1 as f32 * GRID_SIZE as f32,
    )
}

impl Ray2D {
    pub fn get_origin(&self) -> Vec2 {
        self.origin
    }

    pub fn get_angle(&self, transform: &Transform) -> f32 {
        transform.rotation.to_euler(XYZ).2
    }
    ///Retourne le vecteur unitaire de direction
    pub fn get_direction(&self) -> Vec2 {
        self.direction
    }

    pub fn get_end_point(&self) -> Vec2 {
        
        self.origin+self.direction * self.range
            
        
    }

    pub fn get_potential_intersect(&self, grid : &CollisionGrid) -> Vec<Entity>{
        //&self,collision_grid : Res<CollisionGrid>,
        let starting_cell_index = world_to_grid_coordinate(&self.origin);
        let starting_cell_pos = grid_coordinate_to_world(&starting_cell_index);

        let ray_direction = self.get_direction();

        let mut loop_ray_start_point = self.origin;
        let mut loop_cell_pos = starting_cell_pos;
        let mut solution_to_end_point_vector=ray_direction;

        let mut cell_collection: Vec<(i32, i32)> = Vec::default();
        cell_collection.push(starting_cell_index);
        let mut time_out_counter =0;
        let max_cells = max_cells_for_ray(self.range, GRID_SIZE as f32);
        while time_out_counter < 100{

            (loop_ray_start_point, loop_cell_pos) = ray_cell_exit_point(loop_ray_start_point, loop_cell_pos, ray_direction);
            
            
            solution_to_end_point_vector = self.get_end_point() - loop_ray_start_point;
            let solution_to_end_point_vector_normalised = solution_to_end_point_vector.normalize();
            if !check_vector_sign(ray_direction, solution_to_end_point_vector_normalised) {
                break;
            }
         // gizmos.circle_2d(Isometry2d::from_translation(loop_ray_start_point), 10.0, Color::srgb(255.0, 0.0, 255.0));
                 
            cell_collection.push(world_to_grid_coordinate(&loop_cell_pos));
            time_out_counter+=1;
           
        }
         
        if time_out_counter >= 100 {
    println!(
        "WARNING: raycast timeout, position = {:?}, cell = {:?} 
        ray_direction : {:?} == solution_to_end_point_vector : {:?}",
        loop_ray_start_point,
        loop_cell_pos,
        ray_direction,solution_to_end_point_vector
    );
}

        let mut potential_collision_list : Vec<Entity> = Vec::default();
        for cell in cell_collection{

           
            
             if let Some(entities) = get_entity_in_cell(cell, grid){
            for entity in entities.iter(){

                potential_collision_list.push(*entity);
                
            }
        }
        }
         //println!("number : {:?}",potential_collision_list.len());

        potential_collision_list
        
    }
}

fn clip_to_cell_border(local_ray_pos: Vec2) -> Vec2 {
    
    let mut result = local_ray_pos;
    if local_ray_pos.x > GRID_SIZE as f32 {
        result.x = GRID_SIZE as f32;
    }
    if local_ray_pos.x < 0.0 {
        result.x = 0.0;
    }
    if local_ray_pos.y > GRID_SIZE as f32 {
        result.y = GRID_SIZE as f32;
    }
    if local_ray_pos.y < 0.0 {
        result.y = 0.0;
    }

    result
}
fn check_vector_sign(vec1: Vec2, vec2: Vec2) -> bool {
    (vec1 * vec2).x.is_sign_positive() && (vec1 * vec2).y.is_sign_positive()
}

fn get_entity_in_cell(index: (i32,i32),grid : &CollisionGrid)->Option<&EntityHashSet>{

     grid.grid.get(&index)
      
}
fn max_cells_for_ray(ray_length: f32, cell_size: f32) -> usize {
    let max_delta = ray_length / 2.0_f32.sqrt();

    (2.0 * max_delta / cell_size).ceil() as usize + 1
}

fn ray_cell_exit_point(
    ray_start_point: Vec2,
    cell_origin_world: Vec2,
    ray_direction: Vec2,
) -> (Vec2, Vec2) {

  //  let start = Instant::now();
    let mut local_ray_pos = ray_start_point - cell_origin_world;
    local_ray_pos = clip_to_cell_border(local_ray_pos);


    let mut solutions_tab: Vec<(Vec2,f32)> = Vec::new();

    let mut t: f32;
    let mut solution: f32;
    
    //intercetion basse
    if ray_direction.y !=0.0{
    t = (-local_ray_pos.y) / ray_direction.y;
    //println!("exit point : {:?}", t);
    solution = local_ray_pos.x + t * ray_direction.x;
    //println!("solution : {:?}", solution);
    if t>=0.0 {
        solutions_tab.push((vec2(solution, -CELL_BORDER_PADDING),t));
        
    }
    
    //intercetion haute
    t = ((GRID_SIZE as f32) - local_ray_pos.y) / ray_direction.y;
    solution = local_ray_pos.x + t * ray_direction.x;
    if t>=0.0 {
        solutions_tab.push((vec2(solution, GRID_SIZE as f32 + CELL_BORDER_PADDING),t));
    }
    }
    if ray_direction.x !=0.0{
    //intercetion gauche
    t = (-local_ray_pos.x) / ray_direction.x;
    solution = local_ray_pos.y + t * ray_direction.y;
    if t>=0.0 {
        solutions_tab.push((vec2(-CELL_BORDER_PADDING, solution),t));
    }

    //intercetion droite
    t = ((GRID_SIZE as f32) - local_ray_pos.x) / ray_direction.x;
    solution = local_ray_pos.y + t * ray_direction.y;
    if t>=0.0 {
        solutions_tab.push((vec2(GRID_SIZE as f32 + CELL_BORDER_PADDING, solution),t));
    }
}
    //choisi 1 parmis les 4 solution possible en fonction du sens du rayon en coordonée local cellule
    let mut solution_finale: Vec2 = Vec2::default();
    let mut t_temp_minimum = f32::MAX;

    for s in &solutions_tab {
        if s.1 < t_temp_minimum{
            t_temp_minimum =s.1;
            solution_finale = s.0;
        } 
    }

    //coordonée global
    let exit_point = cell_origin_world + solution_finale;
    
    let new_cell = world_to_grid_coordinate(&exit_point);
    let new_cell_world = grid_coordinate_to_world(&new_cell);
   // println!("passé : {:?}",start.elapsed().as_nanos());
    //println!("ancienne cellule : {:?}, nouvelle : {:?}",cell_origin_world,new_cell_world);
    (exit_point,new_cell_world)
    // println!("nombre de solution : {:?}",cell_origin_world+solution_finale);
}

pub fn check_in_cell_or_overlap(position : &Vec2,shape : &ColliderShape)-> Vec<(i32,i32)>{

    let cell_index = world_to_grid_coordinate(position);
    let cell_world_pos = grid_coordinate_to_world(&cell_index);
    let in_cell_local_pos = position-cell_world_pos;

    let mut cell_index_tab :Vec<(i32,i32)> = Vec::default();

    match shape {
        
        ColliderShape::Circle { radius } => {

            let size = GRID_SIZE as f32;

    cell_index_tab.push(cell_index);

    if in_cell_local_pos.x - radius < 0.0 {
        cell_index_tab.push((cell_index.0 - 1, cell_index.1));
    }

    if in_cell_local_pos.x + radius > size {
        cell_index_tab.push((cell_index.0 + 1, cell_index.1));
    }

    if in_cell_local_pos.y - radius < 0.0 {
        cell_index_tab.push((cell_index.0, cell_index.1 - 1));
    }

    if in_cell_local_pos.y + radius > size {
        cell_index_tab.push((cell_index.0, cell_index.1 + 1));
    }

    // Coins
    if in_cell_local_pos.x - radius < 0.0
        && in_cell_local_pos.y - radius < 0.0
    {
        cell_index_tab.push((cell_index.0 - 1, cell_index.1 - 1));
    }

    if in_cell_local_pos.x - radius < 0.0
        && in_cell_local_pos.y + radius > size
    {
        cell_index_tab.push((cell_index.0 - 1, cell_index.1 + 1));
    }

    if in_cell_local_pos.x + radius > size
        && in_cell_local_pos.y - radius < 0.0
    {
        cell_index_tab.push((cell_index.0 + 1, cell_index.1 - 1));
    }

    if in_cell_local_pos.x + radius > size
        && in_cell_local_pos.y + radius > size
    {
        cell_index_tab.push((cell_index.0 + 1, cell_index.1 + 1));
    }

            
            
            
            for  s in &cell_index_tab{
               
               // gizmos.circle_2d(Isometry2d::from_translation(s), 3.0, Color::srgb(255.0, 0.0, 255.0));
                let new_cell_pos= grid_coordinate_to_world(&s);
                //gizmos.rect_2d(Isometry2d::from_translation(new_cell_pos + vec2(50.0, 50.0))  , vec2(100.0, 100.0), Color::srgb(255.0, 0.0, 255.0));
                
               
            }
            if cell_index_tab.is_empty() {
                cell_index_tab.push(cell_index);
               
            }
         
            return cell_index_tab;
        }


        ColliderShape::Rect { size } => {return cell_index_tab;}
    }

    
}

pub struct RayCastResult {
    intersection_list: Vec<RayCastIntersection>
}
pub struct RayCastIntersection{
    entity: Entity,
    distance: f32
}





    
    pub fn cast_rays(source_entity: Entity,ray: &Ray2D, grid: &CollisionGrid,target_query: &Query<(&Transform,&Collider)>) ->RayCastResult{

            let mut result =RayCastResult{intersection_list:Vec::default()};
       
           
            let potential_intersection_list = ray.get_potential_intersect(&grid);
            //for p in potential_intersection{println!("potentiel : {:?}",p);}
             

            for potencial_intersection in potential_intersection_list{
                if potencial_intersection==source_entity{continue;}
                let (target_transform,target_collider) = target_query.get(potencial_intersection).unwrap();
                
               if let Some(intersection) = ray_intersection(ray,target_transform,target_collider){
                //gizmos.circle_2d(Isometry2d::from_translation(intersection.0), 3.0, Color::srgb(0.0, 255.0, 0.0));
                //println!("intersection en : {:?}",intersection.0);
                result.intersection_list.push(RayCastIntersection { entity: potencial_intersection, distance: intersection.1 });
               }
             
            }

            result
    }
        
    

    
   
    

    fn ray_intersection(ray: &Ray2D,target_transform:&Transform,target_collider:&Collider)-> Option<(Vec2,f32)>{
        
        match target_collider.shape{
           
            ColliderShape::Circle { radius } => {
                
               
                let direction = ray.get_direction();
                let m = ray.origin-target_transform.translation.truncate();
                let coef_a = direction.dot(direction);
                let coef_b = 2.0* m.dot(direction);
                let constant_c = m.dot(m)-radius.pow(2.0);
            
                let discriminant = coef_b.pow(2.0)-4.0*(coef_a*constant_c);
                let  s : Vec2;
                if discriminant < 0.0 {return None;}

                if discriminant == 0.0 {
                    let t= (-coef_b)/(2.0*coef_a);
                    if t<0.0 {return None;}
                    s = ray.origin+(direction*t);
                    return Some((s,t));
                }

                else  {
                    let mut t1 = (-coef_b - discriminant.sqrt()) / (2.0 * coef_a);
                    if t1 <0.0 {t1 = f32::MAX}
                    let mut t2 = (-coef_b + discriminant.sqrt()) / (2.0 * coef_a);
                    if t2 <0.0 {t2 = f32::MAX}
                   
                    if t1>ray.range && t2>ray.range{return None;}
                    let distance;
                    if t1<t2 {
                          s = ray.origin+(direction*t1);
                          distance = t1;
                    }
                    else {s = ray.origin+(direction*t2);
                        distance = t2;
                    }
                    
                    return Some((s,distance));
                    
                }
                
            }

            ColliderShape::Rect { size }=>{return None;}
            
        }
        
    }