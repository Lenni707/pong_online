use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod world;
use crate::world::WorldPlugin;

mod paddle;
use crate::paddle::PlayerPaddle;

mod ball;
use crate::ball::BallPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // rapier
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(Startup, set_gravity)

        .add_plugins((WorldPlugin, PlayerPaddle))

        .run();
}

// goofy ashell, weil anscheinend gravity automatisch an ist
fn set_gravity(mut config: Query<&mut RapierConfiguration>) {
    for mut config in config.iter_mut() {
        config.gravity = Vec2::ZERO;
    }
}