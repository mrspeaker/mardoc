use crate::terrain::TerrainPlugin;
use crate::nim::NimPlugin;
use crate::player::PlayerPlugin;
use crate::person::{PersonPlugin,SpawnPerson};

use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;

use rand::prelude::*;

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
        app.add_plugins(PlayerPlugin);
        app.add_plugins(PersonPlugin);
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

fn setup_scene(
    mut commands: Commands,
) {

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-PI / 8.))
    ));

    let mut rng = rand::thread_rng();
    let half = 50.0;
    for _ in 0..20 {
        let pos = Vec3::new(rng.gen_range(-half..half), 0.0, rng.gen_range(-half..half));
        commands.trigger(SpawnPerson { pos, speed: rng.gen_range(0.2..1.2) });
    }
}

fn tag_gltf_heirachy(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    deets: Query<(&GlobalTransform, &Parent, Option<&Name>)>,
) {
    let root = trigger.entity();

    commands.entity(root).insert(GltfLoaded);

    for entity in children.iter_descendants(root) {
        info!("i: {}", entity);
        if let Ok((transform, parent, name)) = deets.get(entity) {
            if let Some(name) = name {
                info!("n: {} {:?} {:?}", name, parent, transform);
                if *name == Name::new("forearm") {
                    commands.entity(entity).insert((Jointy, Timey(0.0)));
                }
                if *name == Name::new("shoulder") {
                    commands.entity(entity).insert((Jointy, Timey(10.0)));
                }
                if *name == Name::new("hand") {
                    commands.entity(entity).insert((Jointy, Timey(3.0)));
                }
            } else {
                info!("t: {:?}", transform);
            }
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
            .normalize() * 2.0;
    }
}

