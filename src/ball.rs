use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball);
        app.add_systems(Update, screen_bounds);
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

        RigidBody::Dynamic,
        Collider::ball(12.5),
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
        Velocity::linear(Vec2::new(-100., 0.)),
        ActiveEvents::COLLISION_EVENTS,
        LockedAxes::ROTATION_LOCKED
    ));
}

fn screen_bounds(
    mut ball_query: Query<&mut Velocity, With<Ball>>,
    ball_transform: Query<&Transform, With<Ball>>,
) {
    let Ok(ball_trans) = ball_transform.single() else { return };
    let Ok(mut velocity) = ball_query.single_mut() else { return };
    
    let pos = ball_trans.translation;
    let mut vel = velocity.linvel;
    
    const SCREEN_WIDTH: f32 = 800.0;
    const SCREEN_HEIGHT: f32 = 600.0;
    const BALL_RADIUS: f32 = 12.5;
    
    if pos.x > SCREEN_WIDTH/2.0 - BALL_RADIUS || pos.x < -SCREEN_WIDTH/2.0 + BALL_RADIUS {
        vel.x *= -1.0;
    }
    if pos.y > SCREEN_HEIGHT/2.0 - BALL_RADIUS || pos.y < -SCREEN_HEIGHT/2.0 + BALL_RADIUS {
        vel.y *= -1.0;
    }
    
    velocity.linvel = vel;
}

