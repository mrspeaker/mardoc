use crate::terrain::TerrainPlugin;
use crate::nim::NimPlugin;
use crate::person::PersonPlugin;

use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;

use std::f32::consts::*;
use std::ops::Add;

pub struct GamePlugin;

#[derive(Component)]
struct Jointy;

#[derive(Component)]
struct Timey(f32);

#[derive(Component)]
struct GltfLoaded;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NimPlugin);
        app.add_plugins(PersonPlugin);
        app.add_plugins(TerrainPlugin);
        app.add_systems(Startup, setup_scene);
        app.add_systems(Update, (update_timers, add_bone_cube, animate_joints));
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

fn setup_scene(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 10., 40.)
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

}

fn tag_gltf_heirachy(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    deets: Query<(&GlobalTransform, &Parent, Option<&Name>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mat = MeshMaterial3d(materials.add(StandardMaterial {
        base_color: Srgba::hex("#ffd891").unwrap().into(),
        ..default()
    }));

    let root = trigger.entity();

    commands.entity(root).insert(GltfLoaded);

    // root transform = transform.
    for entity in children.iter_descendants(root) {
        info!("i: {}", entity);
        if let Ok((transform, parent, name)) = deets.get(entity) {
            if let Some(name) = name {
                info!("n: {} {:?} {:?}", name, parent, transform);
                if *name == Name::new("forearm") {
                    commands.entity(entity).insert((Jointy, Timey(0.0)));
                }
                if *name == Name::new("shoulder") {
                    // flat to start adding to transform from here
                    // do_trans = true...

                    commands.entity(entity).insert((Jointy, Timey(10.0)));
                }
                if *name == Name::new("hand") {
                    commands.entity(entity).insert((Jointy, Timey(3.0)));
                }
            } else {
                info!("t: {:?}", transform);
            }

/*            commands.spawn((
                Mesh3d(meshes.add(Cuboid::default())),
                mat.clone(),
                transform.clone()
            ));*/
        }
    }
}

fn add_bone_cube(
    mut commands: Commands,
    deets: Query<(&Transform, &Parent, Option<&Name>), Added<Jointy>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mat = MeshMaterial3d(materials.add(StandardMaterial {
        base_color: Srgba::hex("#ffd891").unwrap().into(),
        ..default()
    }));

    for (transform, parent, name) in deets.iter() {
        info!("n: {:?} {:?}",  parent, transform);
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::default())),
                mat.clone(),
                transform.clone()//.with_scale(Vec3::new(10.0, 10.0, 10.0))
            ));
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
