use std::f32::consts::PI;

use bevy::{prelude::*, render::mesh::VertexAttributeValues};
use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};
use noise::{NoiseFn, Perlin, Seedable, BasicMulti};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanOrbitCameraPlugin);
        app.add_systems(Startup, setup_scene);
    }
}

#[derive(Component)]
struct Terrain;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 20., 75.)
                .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default()
    ));

    let mut terrain = Mesh::from(
        Plane3d::default()
            .mesh()
            .size(1000., 1000.)
            .subdivisions(200));

    if let Some(VertexAttributeValues::Float32x3(
        positions,
    )) = terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
         let terrain_height = 70.;
        let noise = BasicMulti::<Perlin>::default();

        for pos in positions.iter_mut() {
            let val = noise.get([
                pos[0] as f64 / 300.0,
                pos[2] as f64/ 300.0
            ]);
            pos[1] = val as f32 * terrain_height;
        }
        terrain.compute_normals();
    }

    commands.spawn((
        PbrBundle  {
            mesh: meshes.add(terrain),
            material: materials.add(Color::WHITE),
            ..default()
        },
        Terrain
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });

}
