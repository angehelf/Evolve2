use ocl::{ProQue, Buffer};
use std::{path::Path};
use std::fs;
use std::io::{self};
use crate::matrice::Matrice;

#[macro_export]
macro_rules! kernel_initialize {
($premier:expr $(, $autres:expr)* $(,)? ) => {{
        use std::path::Path;
    
        let p : &Path = &Path::new($premier);
        let f: &[&Path] = &[$(Path::new($autres)),*];
        
      
  kernel_initialize_real(p,f)

}};



}



 pub fn kernel_initialize_real(premier: &Path,fichiers: &[&Path]) ->io::Result<ProQue>{
let mut pro_que :ProQue;
let mut entire_kernel_code : String = "".to_string();
   
    if let Some(chemin) = premier.to_str(){

            match fs::read_to_string(chemin){

                Ok(contenu)=>{
                    entire_kernel_code.push_str("\n");
                    entire_kernel_code.push_str(&contenu);
                   
                },
                Err(_) =>{

                    println!("le fichier: {} n'a pas été trouvé au chemin spécifié, code non ajouté au kernel.",chemin);
                }
            }
        } 
    
      
    for p in fichiers {
    if let Some(chemin) = p.to_str(){

            match fs::read_to_string(chemin){

                Ok(contenu)=>{
                    entire_kernel_code.push_str("\n");
                    entire_kernel_code.push_str(&contenu);
                },
                Err(_) =>{

                    println!("le fichier: {} n'a pas été trouvé au chemin spécifié, code non ajouté au kernel.",chemin);
                }
            }
        } 
    }



if(entire_kernel_code.is_empty()){
return Err(io::Error::new(io::ErrorKind::InvalidInput,"Pas de code kernel validé"));
}
 
 match ProQue::builder().src(entire_kernel_code).build() {
    Ok(pro_que) =>{
    println!("contexte OPEN CL créé avec succes");
       return  Ok(pro_que);

    },
         
    Err(e) => {
        println!("une erreur c'est produite lors de la compilation du kernel, erreur : {}",e);
        Err(io::Error::new(io::ErrorKind::Other," Echec de la creattion du ProQue"))
    }
     
 }
 

}





 fn find_local_size(dimx : usize , dimy: usize)-> (usize , usize, usize) {
   
   let mut max_div_x = 1;
   let mut max_div_y = 1;
   for i in 1..16{
    if dimx % i ==0 {
    max_div_x = i;
    }
    if dimy % i ==0 {
    max_div_y = i;
    }
   }

   let k = std::cmp::min(max_div_x, max_div_y);

(max_div_x,max_div_y,k)



}








pub trait ParallelMatMult<T : Default + Clone> : Sized {

fn parallel_mat_mult(m_a : &Matrice<T>, m_b : &Matrice<T> ,pro_que: &ProQue)-> Matrice<T>;
fn parallel_mat_mult_small(m_a : &Matrice<T>, m_b : &Matrice<T> ,pro_que: &ProQue)-> Matrice<T>;
   
}


impl ParallelMatMult<i32> for Matrice<i32> {
    fn parallel_mat_mult(m_a: &Matrice<i32>, m_b: &Matrice<i32>,pro_que: &ProQue) -> Matrice<i32> {
    let size = m_a.height * m_b.width;
    

    let a = Buffer::<i32>::builder()
        .queue(pro_que.queue().clone())
        .len(m_a.data.len())
        .copy_host_slice(&m_a.data)
        .build().unwrap();

    let b = Buffer::<i32>::builder()
        .queue(pro_que.queue().clone())
        .len(m_b.data.len())
        .copy_host_slice(&m_b.data)
        .build().unwrap();

    let result = Buffer::<i32>::builder()
        .queue(pro_que.queue().clone())
        .len(size)
        .build().unwrap();

        let mut res:Matrice<i32> = Matrice::new(m_a.height, m_b.width);

        let local_size :(usize,usize,usize) = find_local_size(res.height, res.width);
      
        
    let kernel = pro_que.kernel_builder("int_mat_mult_large")
        .arg(&a)
        .arg(&b)
        .arg(&result)
        .arg(m_a.width as i32)
        .arg(m_a.height as i32)
        .arg(m_b.width as i32)
        .arg_local::<i32>(local_size.0* local_size.2 * size_of::<i32>())
        .arg_local::<i32>(local_size.1* local_size.2 * size_of::<i32>())
        .arg(local_size.0 as i32)
        .arg(local_size.1 as i32)
        .arg(local_size.2 as i32)
        .build().unwrap();
       // println!("taille d'un tile: {:?}",local_size);
     
     
    unsafe { kernel.cmd()
        .global_work_size([m_a.height,m_b.width])
        .local_work_size(local_size)
        .enq().unwrap(); }

    
    
   
    result.read(&mut res.data).enq().unwrap();
    res
    }




    fn parallel_mat_mult_small(m_a : &Matrice<i32>, m_b : &Matrice<i32> ,pro_que: &ProQue)-> Matrice<i32>{
         let size = m_a.height * m_b.width;
    

    let a = Buffer::<i32>::builder()
        .queue(pro_que.queue().clone())
        .len(m_a.data.len())
        .copy_host_slice(&m_a.data)
        .build().unwrap();

    let b = Buffer::<i32>::builder()
        .queue(pro_que.queue().clone())
        .len(m_b.data.len())
        .copy_host_slice(&m_b.data)
        .build().unwrap();

    let result = Buffer::<i32>::builder()
        .queue(pro_que.queue().clone())
        .len(size)
        .build().unwrap();

        let mut res:Matrice<i32> = Matrice::new(m_a.height, m_b.width);

        
      
        
    let kernel = pro_que.kernel_builder("int_mat_mult_small")
        .arg(&a)
        .arg(&b)
        .arg(&result)
        .arg(m_a.width as i32)
        .arg(m_a.height as i32)
        .build().unwrap();
      
     
     
    unsafe { kernel.cmd()
        .global_work_size([m_a.height,m_b.width])
        .enq().unwrap(); }

    
    
   
    result.read(&mut res.data).enq().unwrap();
    res


    }

}




impl ParallelMatMult<f32> for Matrice<f32> {
    fn parallel_mat_mult(m_a: &Matrice<f32>, m_b: &Matrice<f32>,pro_que: &ProQue) -> Matrice<f32> {
    let size = m_a.height * m_b.width;
    

    let a = Buffer::<f32>::builder()
        .queue(pro_que.queue().clone())
        .len(m_a.data.len())
        .copy_host_slice(&m_a.data)
        .build().unwrap();

    let b = Buffer::<f32>::builder()
        .queue(pro_que.queue().clone())
        .len(m_b.data.len())
        .copy_host_slice(&m_b.data)
        .build().unwrap();

    let result = Buffer::<f32>::builder()
        .queue(pro_que.queue().clone())
        .len(size)
        .build().unwrap();

        let mut res:Matrice<f32> = Matrice::new(m_a.height, m_b.width);

        let local_size :(usize,usize,usize) = find_local_size(res.height, res.width);
      
        
    let kernel = pro_que.kernel_builder("f32_mat_mult_large")
        .arg(&a)
        .arg(&b)
        .arg(&result)
        .arg(m_a.width as i32)
        .arg(m_a.height as i32)
        .arg(m_b.width as i32)
        .arg_local::<f32>(local_size.0* local_size.2 * size_of::<f32>())
        .arg_local::<f32>(local_size.1* local_size.2 * size_of::<f32>())
        .arg(local_size.0 as i32)
        .arg(local_size.1 as i32)
        .arg(local_size.2 as i32)
        .build().unwrap();
       // println!("taille d'un tile: {:?}",local_size);
     
     
    unsafe { kernel.cmd()
        .global_work_size([m_a.height,m_b.width])
        .local_work_size(local_size)
        .enq().unwrap(); }

    
    
   
    result.read(&mut res.data).enq().unwrap();
    res
    }




    fn parallel_mat_mult_small(m_a : &Matrice<f32>, m_b : &Matrice<f32> ,pro_que: &ProQue)-> Matrice<f32>{
         let size = m_a.height * m_b.width;
    

    let a = Buffer::<f32>::builder()
        .queue(pro_que.queue().clone())
        .len(m_a.data.len())
        .copy_host_slice(&m_a.data)
        .build().unwrap();

    let b = Buffer::<f32>::builder()
        .queue(pro_que.queue().clone())
        .len(m_b.data.len())
        .copy_host_slice(&m_b.data)
        .build().unwrap();

    let result = Buffer::<f32>::builder()
        .queue(pro_que.queue().clone())
        .len(size)
        .build().unwrap();

        let mut res:Matrice<f32> = Matrice::new(m_a.height, m_b.width);

        
      
        
    let kernel = pro_que.kernel_builder("f32_mat_mult_small")
        .arg(&a)
        .arg(&b)
        .arg(&result)
        .arg(m_a.width as i32)
        .arg(m_a.height as i32)
        .build().unwrap();
      
     
     
    unsafe { kernel.cmd()
        .global_work_size([m_a.height,m_b.width])
        .enq().unwrap(); }

    
    
   
    result.read(&mut res.data).enq().unwrap();
    res
















    }

}