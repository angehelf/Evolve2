const ALPHA:f32 = 0.1;

#[inline]
pub fn sigmoid(x:f32)->f32{

      1.0/(1.0 + (-x).exp())

}

pub fn d_sigmoid(x:f32)->f32{

sigmoid(x)*(1.0-sigmoid(x))
}



pub fn p_relu(x:f32) ->f32{

if x >= 0.0 {
    return x;
}
else {
    return ALPHA*x;
}

}
///alpha doit etre égal au alpha de p_relu pour une dérivée valide
pub fn d_p_relu(x:f32)->f32{
if x >=0.0 {
    return 1.0 ;
}
else {
    return ALPHA;
}

}

pub fn tanh(x:f32)->f32{

    x.tanh()

}

pub fn d_tanh(x:f32)->f32{


    1.0- (x.tanh()* x.tanh())
}