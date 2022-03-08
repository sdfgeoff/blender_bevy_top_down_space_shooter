use bevy::prelude::*;


#[derive(Reflect, Default, Component, Debug)]
#[reflect(Component)]
pub struct Engine {
    pub lin_drag: Vec3,
    pub ang_drag: Vec3,
    pub lin_thrust: Vec3,
    pub ang_thrust: Vec3,
}