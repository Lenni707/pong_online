use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPaddle;
impl Plugin for PlayerPaddle{
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_paddle)
            .add_systems(Update, move_paddle);
    }
}

#[derive(Component)]
struct Paddle;

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
        RigidBody::Dynamic,
        Collider::cuboid(10.0, 50.0),
        Velocity::zero(),
        ActiveEvents::COLLISION_EVENTS,
        LockedAxes::ROTATION_LOCKED
    ));
}

const PADDLE_SPEED: f32 = 600.0;

fn move_paddle(
    mut q: Query<&mut Velocity, With<Paddle>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for mut paddel in &mut q {
        let mut dir = 0.0;
        if keyboard.pressed(KeyCode::KeyW) { dir += 1.0; }
        if keyboard.pressed(KeyCode::KeyS) { dir -= 1.0; }

        paddel.linvel = Vec2::new(0.0, dir * PADDLE_SPEED);
    }
}




