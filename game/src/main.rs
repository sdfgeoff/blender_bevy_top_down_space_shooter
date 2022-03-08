use bevy::input::{keyboard::KeyboardInput, ElementState};
use bevy::prelude::*;
use bevy_rapier3d::physics::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier3d::prelude::*;
use blender_bevy_toolkit::BlendLoadPlugin;

mod ship;
mod player;




fn start_game(
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,){
    println!("Loading Level 1");
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


fn control_ship(
        mut playable_ships_query: Query<(&player::Player, &ship::Ship, &mut RigidBodyForcesComponent, &mut RigidBodyVelocityComponent, &RigidBodyMassPropsComponent)>,
    ) {
    for (player, ship, mut rb_forces, mut rb_velocities, rb_mprops) in playable_ships_query.iter_mut() {

        let mut forces = Vec3::new(0.0, 0.0, 0.0);
        // Thrust
        for action in player.actions.iter() {
            forces += action.as_lin_vec() * ship.thrust;
        }

        // Drag
        let linvel: Vec3 = rb_velocities.linvel.into();
        forces -= linvel * ship.drag;


        rb_forces.force = forces.into();

        println!("{:?}, {:?}", player, ship);
    }
}








fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(BlendLoadPlugin::default())
        .register_type::<ship::EngineFlare>()
        .register_type::<ship::Ship>()
        .register_type::<player::Player>()
        .add_startup_system(start_game)
        .add_startup_system(setup_world)

        .add_system(player::read_input.label("input"))
        .add_system(control_ship.after("input"))

        .run();
}

