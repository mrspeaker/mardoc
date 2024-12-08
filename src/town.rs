use crate::terrain::TerrainPlugin;
use rand::prelude::*;

use std::f32::consts::*;

use bevy::prelude::*;

pub struct TownPlugin;

impl Plugin for TownPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TerrainPlugin);
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();

    commands
        .spawn((
            Name::new("building1"),
            SceneRoot(
                asset_server
                    .load(GltfAssetLabel::Scene(0).from_asset("building1.glb"))),
            Transform::from_xyz(30.0, 0.0, 20.0)
                .with_rotation(Quat::from_rotation_y((PI / 2.) * 2.0))
                .with_scale(Vec3::splat(1.0))
        ));

    commands
        .spawn((
            Name::new("building2"),
            SceneRoot(
                asset_server
                    .load(GltfAssetLabel::Scene(0).from_asset("building1.glb"))),
            Transform::from_xyz(0.0, 0.0, -60.0)
                .with_rotation(Quat::from_rotation_y(PI * 2.0))
                .with_scale(Vec3::splat(1.0))
        ));


    commands
        .spawn((
            Name::new("home"),
            SceneRoot(
                asset_server
                    .load(GltfAssetLabel::Scene(0).from_asset("home.glb"))),
            Transform::from_xyz(0.0, 0.0, 40.0)
                //.with_rotation(Quat::from_rotation_y(PI / 2.))
                .with_scale(Vec3::splat(1.0))
        ));


    let half = 40.0;
    for _ in 0..20 {
        let pos = Vec3::new(rng.gen_range(-half..half), 0.0, rng.gen_range(-half..half));
        //commands.trigger(SpawnPerson { pos, speed: rng.gen_range(0.2..1.2) });
        commands
            .spawn((
                Name::new("tree"),
                SceneRoot(
                    asset_server
                        .load(GltfAssetLabel::Scene(0).from_asset("tree.glb"))),
                Transform::from_xyz(pos.x, pos.y, pos.z)
                //.with_rotation(Quat::from_rotation_y(PI / 2.))
                    .with_scale(Vec3::splat(1.0))
            ));

    }



    let mat = MeshMaterial3d(materials.add(StandardMaterial {
        base_color: Srgba::hex("#443333").unwrap().into(),
        ..default()
    }));

    commands.spawn((
        Name::new("Scale"),
        Mesh3d(meshes.add(Cuboid::new(2.5, 2.5, 0.2))),
        mat.clone(),
        Transform::from_xyz(0.0, 1.25, 0.0),
    ));

    commands.spawn((
        Name::new("Liney"),
        Mesh3d(meshes.add(Cuboid::new(0.1, 0.1, 50.0))),
        mat.clone(),
        Transform::from_xyz(0.0, 0.0, -25.0),
    ));


}
