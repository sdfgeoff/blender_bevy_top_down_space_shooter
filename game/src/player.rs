use bevy::prelude::*;

#[derive(Debug)]
pub enum Action {
    StrafeForwards(f32),
    StrafeBackwards(f32),
    StrafeLeft(f32),
    StrafeRight(f32),
    TurnLeft(f32),
    TurnRight(f32),
}

impl Action {
    pub fn as_lin_vec(&self) -> Vec3 {
        match self {
            Self::StrafeForwards(p) => Vec3::new(0.0, *p, 0.0),
            Self::StrafeBackwards(p) => Vec3::new(0.0, -*p, 0.0),
            Self::StrafeLeft(p) => Vec3::new(-*p, 0.0, 0.0),
            Self::StrafeRight(p) => Vec3::new(*p, 0.0, 0.0),
            _ => Vec3::new(0.0, 0.0, 0.0),
        }
    }
    pub fn as_ang_vec(&self) -> Vec3 {
        match self {
            Self::TurnLeft(p) => Vec3::new(0.0, 0.0, *p),
            Self::TurnRight(p) => Vec3::new(0.0, 0.0, -*p),
            _ => Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Reflect, Default, Component, Debug)]
#[reflect(Component)]
pub struct Player {
    #[reflect(ignore)]
    pub actions: Vec<Action>,
}

pub fn read_input(mut query: Query<&mut Player>, keys: Res<Input<KeyCode>>) {
    for mut player in query.iter_mut() {
        player.actions.clear();

        if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Up) {
            player.actions.push(Action::StrafeForwards(1.0))
        }
        if keys.pressed(KeyCode::S) || keys.pressed(KeyCode::Down) {
            player.actions.push(Action::StrafeBackwards(1.0))
        }
        if keys.pressed(KeyCode::A) {
            player.actions.push(Action::StrafeLeft(1.0))
        }
        if keys.pressed(KeyCode::D) {
            player.actions.push(Action::StrafeRight(1.0))
        }

        if keys.pressed(KeyCode::Left) {
            player.actions.push(Action::TurnLeft(1.0))
        }
        if keys.pressed(KeyCode::Right) {
            player.actions.push(Action::TurnRight(1.0))
        }
    }
}
