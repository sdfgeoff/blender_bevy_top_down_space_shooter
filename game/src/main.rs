use bevy::prelude::*;
use bevy_rapier3d::physics::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier3d::prelude::*;
use blender_bevy_toolkit::BlendLoadPlugin;

mod player;
mod ship;

fn start_game(asset_server: Res<AssetServer>, mut scene_spawner: ResMut<SceneSpawner>) {
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




fn control_ship(
    mut playable_ships_query: Query<(
        &player::Player,
        &ship::Ship,
        &GlobalTransform,
        &mut RigidBodyForcesComponent,
        &RigidBodyVelocityComponent,
    )>,
) {
    for (player, ship, global_transform, mut rb_forces, rb_velocities) in
        playable_ships_query.iter_mut()
    {

        let mut forces = Vec3::new(0.0, 0.0, 0.0);
        let mut torques = Vec3::new(0.0, 0.0, 0.0);

        // Thrust
        for action in player.actions.iter() {
            forces += action.as_lin_vec() * ship.thrust;
            torques += action.as_ang_vec() * ship.thrust;
        }
        // Thrust is in local-space
        let local_to_global = global_transform.compute_matrix();
        forces = local_to_global.transform_vector3(forces);
        torques = local_to_global.transform_vector3(torques);


        // Drag
        let linvel: Vec3 = rb_velocities.linvel.into();
        forces -= linvel * ship.drag;
        let angvel: Vec3 = rb_velocities.angvel.into();
        torques -= angvel * ship.drag;

        rb_forces.force = forces.into();
        rb_forces.torque = torques.into();
    }
}