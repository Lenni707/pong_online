use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPaddle;
impl Plugin for PlayerPaddle {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_paddle)
            .add_systems(Update, (move_paddle, clamp_paddle));
    }
}

#[derive(Component)]
pub struct Paddle;

fn spawn_paddle(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    cmd.spawn((
        Mesh2d(meshes.add(Rectangle::new(20.0, 100.0))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(-300., 0., 1.),
        Paddle,

        // Rapier
        RigidBody::KinematicPositionBased,
        Collider::cuboid(10.0, 50.0),
        ActiveEvents::COLLISION_EVENTS,
        LockedAxes::ROTATION_LOCKED,
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
    ));
}

const PADDLE_SPEED: f32 = 600.0;

fn move_paddle(
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in &mut paddle_query {
        let mut dir = 0.0;
        if keyboard.pressed(KeyCode::KeyW) { dir += 1.0; }
        if keyboard.pressed(KeyCode::KeyS) { dir -= 1.0; }
        transform.translation.y += dir * PADDLE_SPEED * time.delta_secs();
    }
}

fn clamp_paddle(
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
) {
    const SCREEN_HEIGHT: f32 = 600.0;
    const PADDLE_HALF_HEIGHT: f32 = 50.0;
    
    for mut transform in &mut paddle_query {
        let clamped_y = transform.translation.y.clamp(
            -SCREEN_HEIGHT/2.0 + PADDLE_HALF_HEIGHT,
            SCREEN_HEIGHT/2.0 - PADDLE_HALF_HEIGHT
        );
        transform.translation.y = clamped_y;
    }
}
