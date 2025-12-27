use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin{
    fn build(&self, app: &mut App){
        app.insert_resource(ClearColor(Color::BLACK));
        app.add_systems(Startup, init_cam);
    }
}

pub fn init_cam(
    mut commands: Commands,
) {
    commands.spawn(
        Camera2d
    );
}