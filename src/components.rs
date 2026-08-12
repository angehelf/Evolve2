
use bevy::{prelude::*};

#[derive(Component)]
pub struct Size(pub f32);

#[derive(Component)]
pub struct Speed(pub Vec2);

impl Default for Speed{
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

#[derive(Component)]
pub struct Heading(pub f32);

#[derive(Component)]
pub struct HeadingSpeed(pub f32);

#[derive(Component)]
pub struct Healt(pub f32);

impl Default for Healt{

    fn default() -> Self {
      Self(0.0)
    }
}

#[derive(Component)]
pub struct Bouncy{
    restitution_coef : f32
}
impl Default for Bouncy{

    fn default()-> Self{
        Self{
            restitution_coef : 1.0
        }
    }
}

#[derive(Component)]
pub struct Selected;