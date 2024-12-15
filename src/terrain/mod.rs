use bevy::{prelude::*, render::mesh::VertexAttributeValues};
use noise::{NoiseFn, Perlin, BasicMulti};
use crate::person::{Pickable};

#[derive(Component)]
pub struct Terrain;

#[derive(Resource)]
struct GreetTimer(Timer);

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, setup);
        app.add_systems(Update, greet_terrain);
    }
}

fn greet_terrain(time: Res<Time>, mut timer: ResMut<GreetTimer>, _query: Query<&Terrain>) {
    if timer.0.tick(time.delta()).just_finished() {
//        println!("hello");
    }
}

fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    return (dx * dx + dy * dy).sqrt();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>) {

    let mat = MeshMaterial3d(materials.add(StandardMaterial {
        base_color: Srgba::hex("#668855").unwrap().into(),
        perceptual_roughness: 0.2,
        reflectance: 0.0,
        ..default()
    }));

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
                pos[2] as f64 / 300.0
            ]);
            let d = dist(0 as f64, 0 as f64, pos[0] as f64, pos[2] as f64);
            let mult = ((d - 50.0) / 50.0).clamp(0.0, 1.0);
            pos[1] = (val * mult) as f32 * terrain_height;
        }
        terrain.compute_normals();
    }

    commands.spawn((
        Mesh3d(meshes.add(terrain)),
        mat,
        Terrain,
    ));
}
