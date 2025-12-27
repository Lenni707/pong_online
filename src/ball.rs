use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball);
    }
}

#[derive(Component)]
struct Ball;

fn spawn_ball(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    cmd.spawn((
        Mesh2d(meshes.add(Circle::new(12.5))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0., 0., 1.),
        Ball,

        // Rapier
        RigidBody::Dynamic,
        Collider::ball(12.5),
        Velocity::zero(),
        ActiveEvents::COLLISION_EVENTS,
        LockedAxes::ROTATION_LOCKED
    ));
}

fn move_ball(
    mut ball_query: Query<(&mut Velocity, &Ball)>,
    
)
