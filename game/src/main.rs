use bevy::input::{keyboard::KeyboardInput, ElementState};
use bevy::prelude::*;
use bevy_rapier3d::physics::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier3d::prelude::*;
use blender_bevy_toolkit::BlendLoadPlugin;

mod ship;

use ship::Engine;


fn start_game( mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,){
    println!("Loading Start");
    let scene_handle: Handle<DynamicScene> = asset_server.load("scenes/Level1.scn");
    scene_spawner.spawn_dynamic(scene_handle);
}


fn setup_world(
    mut physics_config: ResMut<RapierConfiguration>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    ambient_light.color = Color::BLACK;
    physics_config.gravity.y = 0.0;
    physics_config.gravity.z = -9.8;
}




fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(BlendLoadPlugin::default())
        .register_type::<Engine>()
        .add_startup_system(start_game)
        .add_startup_system(setup_world)

        .run();
}

