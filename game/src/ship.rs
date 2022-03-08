use bevy::prelude::*;


#[derive(Reflect, Default, Component, Debug)]
#[reflect(Component)]
pub struct Ship {
    pub thrust: Vec3,
    pub drag: Vec3,
}

#[derive(Reflect, Default, Component, Debug)]
#[reflect(Component)]
pub struct EngineFlare {

}

