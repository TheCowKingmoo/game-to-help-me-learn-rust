use bevy::prelude::*;

mod iso_camera;
mod map;
mod map_lighting;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(iso_camera::pan_orbit_camera)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    map_lighting::generate_lighting(&mut commands);

    // create the grid
    map::create_plane(&mut commands, meshes, materials);

    iso_camera::spawn_camera(&mut commands);
}
