use bevy::{prelude::{Commands, Transform, default}, pbr::PointLightBundle};


pub fn generate_lighting(
    commands: &mut Commands
)  {
        // light
        commands.spawn_bundle(PointLightBundle {
            transform: Transform::from_xyz(3.0, 8.0, 5.0),
            ..default()
        });
}