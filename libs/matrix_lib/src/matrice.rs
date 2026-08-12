#![allow(unused)]

use std::ops::*;

use num_traits::{NumCast, Float, PrimInt};
use utilities_lib::math_utils;
use rand::*;
use rand_distr::{Normal,Distribution};

#[derive(Clone,Default)]
pub struct Matrice<Type: Default + Clone>{
    pub height : usize,
    pub width :usize,
    pub data: Vec<Type>,

    

}





impl<Type: Default + Clone + ToString > Matrice<Type>
where
    Type : Add<Output = Type>,
    Type: AddAssign,
    Type: SubAssign,
    Type : Clone,
    Type : Mul<Output = Type>,
    Type: MulAssign,
    Type : std::fmt::Debug,
    Type: Copy + NumCast,
    Type: PartialEq,
    Matrice<Type>: Clone,
     Type: DivAssign,
     Type: PartialOrd,
    {
    

    pub fn test_equal(a:&Matrice<Type>,b : &Matrice<Type>)->bool {
        let  mut res : bool = false;
        if a.height == b.height && a.width==b.width {
            for ind in 0..(a.height*a.width){
                    if a.data[ind] == b.data[ind]{
                    res = true;
                    } 
                    else {res = false;}
            }

        }
    res
    }
    pub fn new(height : usize,width :usize) -> Self{

        let data :Vec<Type> = vec![Type::default();height*width];
        Self {height, width,data }
    }
    pub fn randomize_int(&mut self,range : i32){
        let mut rng = rand::rng();
        for d in &mut self.data{
            let r: i32 = rng.random_range(0..range);
            *d = NumCast::from(r).unwrap();
        }

    }

      pub fn randomize_f32(&mut self,range : (f32,f32)){
        let mut rng = rand::rng();
        for d in &mut self.data{

            let r: f32 = rng.random_range(range.0..range.1);
            *d = NumCast::from(r).unwrap();
        }

    }

    pub fn normal_distr(&mut self,moyenne:f32,ecart_type:f32){
        let mut rng = rand::rng();
        let normal = Normal::new(moyenne,ecart_type);

        for d in &mut self.data{

            let r: f32 = normal.unwrap().sample(&mut rng);
            *d = NumCast::from(r).unwrap();
        }

    }

    

    pub fn fill(&mut self,value : Type)
    {
        for val in self.data.iter_mut(){

            *val = value.clone();
        }
    }
    pub fn print(&self){
        print!("\n");
       
        for j in 0..self.height{
            for i in 0..self.width{

               print!("{} ",self.data[i*self.height + j].to_string());
                
            }
            print!("\n");
            
        }
        
            print!("\n");
    }

pub fn set(&mut self,i:usize,j :usize, value : Type){

self.data[(j*self.height) + i] = value;

}
pub fn get(&self,i:usize,j :usize)->Type
{
    
self.data[(j*self.height) + i].clone()
}
pub fn add_at_location(&mut self,i:usize,j:usize,value : Type)
{

    self.data[(j*self.height) + i] += value;
}


pub fn add(&mut self, matrice : &Matrice<Type>)->Matrice<Type>
{
    
let mut res = self.clone();
    if self.data.len() == matrice.data.len(){
        
    for index in 0..res.data.len(){
        res.data[index] += matrice.data[index].clone();
    }
}
res
}



pub fn sub(&mut self, matrice : &Matrice<Type>)

{
    if self.data.len() == matrice.data.len(){
    for index in 0..self.data.len(){
        self.data[index] -= matrice.data[index].clone();
    }
}

}

pub fn scale(& self, scalar: Type)->Matrice<Type>

{
    let mut res = self.clone();
    for index in 0..self.data.len(){
        res.data[index] *=scalar;
    }
res

}

pub fn div(& self, scalar: Type)->Matrice<Type>

{
    let mut res = self.clone();
    for index in 0..self.data.len(){
        res.data[index] /=scalar;
    }
res

}

pub fn add_to_all(& self, scalar: Type)->Matrice<Type>

{
    let mut res = self.clone();
    for index in 0..self.data.len(){
        res.data[index] +=scalar;
    }
res

}
pub fn pair_mult(&mut self, matrice : &Matrice<Type>)

{
    if self.data.len() == matrice.data.len(){
    for index in 0..self.data.len(){
        self.data[index] *= matrice.data[index].clone();
    }
}

}




pub fn mat_mult(a:&Matrice<Type>,b : &Matrice<Type>)-> Matrice<Type>{
    let mut res: Matrice<Type> = Matrice::new(a.height, b.width);
    //println!("res : {} {}", res.height,res.width);
    if a.width == b.height{
  

  for j in 0..res.width{
    for i in 0..res.height{

       
        for k in 0..a.width{
        //print!("i: {}, j: {}, k: {}, \n",i,j,k);
        
        res.add_at_location(i, j, a.get(i,k)*b.get(k,j));

  }}}
 // println!("{:?}",res.data);
    res
}
else {
     println!("taille des matrice incompatible, résultat non calculer");
     res
     
}
}


pub fn transpose(& self)-> Matrice<Type> {
    let mut buffer :Matrice<Type> = Matrice::new(self.width, self.height);
    
    for j in 0..self.width  {   
        for i in 0..self.height  {
        
            buffer.set(j,i,self.get(i, j));
        
    }
    }

     buffer


}
    /// insert A dans B a une position h ,w (B doit etre plus grand que A + les positions sur les deux dimensions)
    pub fn insert_matrice(A: &Matrice<Type>,B: &Matrice<Type>,h : usize , w: usize)-> Matrice<Type>{
        let mut  res = B.clone();
        if B.height >= A.height+h && B.width >= A.width+w  {

            for j in 0..A.width{
                 for i in 0..A.height{

                    res.set(i+h, j+w, A.get(i, j));


                 }
                
            }
            }
        

        res
    }


    pub fn concatenate_vec(A: &Matrice<Type>,B: &Matrice<Type>)-> Matrice<Type>{
        let mut  res: Matrice<Type> = Matrice::new(A.height+B.height, 1);
        if A.width ==1 && B.width == 1{

            for i in 0..B.data.len(){
                res.data[i] = B.data[i];

            }

             for i in 0..A.data.len(){
                res.data[i+B.height] = A.data[i];

            }




        }
        
        res

    }





    
}




impl Matrice<f32>  {
        

        pub fn p_relu(a : &Matrice<f32>)-> Matrice<f32>{
        let mut res = a.clone();
        for d in res.data.iter_mut(){

        *d = math_utils::p_relu(*d) ;

        }
        res
    }   

     pub fn d_p_relu(a : &Matrice<f32>)->Matrice<f32>{
        let mut res = a.clone();
        for d in res.data.iter_mut(){

        *d = math_utils::d_p_relu(*d) ;

        
        }
        res

    }   

     pub fn sigmoid( a : &Matrice<f32>)-> Matrice<f32>{
    let mut res = a.clone();
        for d in res.data.iter_mut(){

        *d = math_utils::sigmoid(*d) ;

        }
    res
    }   
    pub fn d_sigmoid(a : &Matrice<f32>)->Matrice<f32>{
        let mut res = a.clone();
        for d in res.data.iter_mut(){

        *d = math_utils::d_sigmoid(*d) ;

        }
        res

    }   

    pub fn tanh(a : &Matrice<f32>)->Matrice<f32>{
        let mut res = a.clone();
        for d in res.data.iter_mut(){

        *d = math_utils::tanh(*d) ;

        }
        res

    }   

        pub fn d_tanh(a : &Matrice<f32>)->Matrice<f32>{
        let mut res = a.clone();
        for d in res.data.iter_mut(){

        *d = math_utils::d_tanh(*d) ;

        }
        res

    } 

        pub fn linear(a : &Matrice<f32>)->Matrice<f32>{
   
            a.clone()
        } 
        
        pub fn d_linear(a : &Matrice<f32>)->Matrice<f32>{
            
            let mut res = a.clone();
            res.fill(1.0);
            res
            
        } 




///retourne une matrice de norme égale à la valeur max (négative ou positive)
///max est est en grandeur absolue
    pub fn normalize(&self, max: f32)->Matrice<f32>

{
    let mut res = self.clone();
    let mut norm =0.0;
    
    for index in 0..res.data.len(){
        norm += res.data[index] * res.data[index];
    }
     norm = norm.sqrt();

     if norm != 0.0{
    for index in 0..res.data.len(){
       res.data[index] = res.data[index] / norm;
       res.data[index] = res.data[index] * max;
    }
}

res

}
}
