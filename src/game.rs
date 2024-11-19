use crate::terrain::TerrainPlugin;
use crate::nim::NimPlugin;

use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;

use std::f32::consts::*;
use std::ops::Add;

pub struct GamePlugin;

#[derive(Component)]
struct Jointy;

#[derive(Component)]
struct Timey(f32);

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NimPlugin);
        app.add_plugins(TerrainPlugin);
        app.add_systems(Startup, setup_scene);
        app.add_systems(Update, (update_timers, animate_joints));
        app.add_observer(tag_gltf_heirachy);
    }
}

fn update_timers(
    time: Res<Time>,
    mut timer: Query<&mut Timey>
) {
    for mut t in timer.iter_mut() {
        t.0 += time.delta_secs();
    }
}

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(20., 10., 20.)
            .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-PI / 8.))
    ));

    commands
        .spawn((SceneRoot(
            asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("arm.glb"))),
            Transform::from_xyz(0.0, 0.0, 0.0)
                .with_scale(Vec3::new(10.0, 10.0, 10.0))
        ));

    commands
        .spawn((SceneRoot(
            asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("arm.glb"))),
            Transform::from_xyz(10.0, 0.0, -10.0)
                .with_scale(Vec3::new(10.0, 10.0, 10.0))
        ));

}

fn tag_gltf_heirachy(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    deets: Query<(&GlobalTransform, Option<&Name>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mat = MeshMaterial3d(materials.add(StandardMaterial {
        base_color: Srgba::hex("#ffd891").unwrap().into(),
        ..default()
    }));

    // root transform = transform.
    for entity in children.iter_descendants(trigger.entity()) {
        info!("i: {}", entity);
        if let Ok((transform, name)) = deets.get(entity) {
            if let Some(name) = name {
                info!("n: {} {:?}", name, transform);
                if *name == Name::new("forearm") {
                    commands.entity(entity).insert((Jointy, Timey(0.0)));
                }
                if *name == Name::new("shoulder") {
                    // flat to start adding to transform from here
                    // do_trans = true...

                    commands.entity(entity).insert((Jointy, Timey(10.0)));
                }
                if *name == Name::new("hand") {
                    commands.entity(entity).insert((Jointy, Timey(5.0)));
                }
            } else {
                info!("t: {:?}", transform);
            }

            commands.spawn((
                Mesh3d(meshes.add(Cuboid::default())),
                mat.clone(),
                transform.clone()
            ));
        }
    }
}

fn animate_joints(
    mut joints: Query<(&mut Transform, &Timey), With<Jointy>>,
) {
    for (mut t, timey) in joints.iter_mut() {
        let sec = timey.0;
        t.rotation =
            Quat::from_rotation_y(FRAC_PI_2 * sec.sin() * 0.5)
            .add(Quat::from_rotation_z(FRAC_PI_2 * sec.cos() * 0.4))
            .normalize();
    }
}
