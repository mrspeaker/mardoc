use crate::terrain::TerrainPlugin;
use crate::nim::NimPlugin;
use crate::player::{PlayerPlugin,Player};
use crate::person::{Pickable,PersonPlugin,SpawnPerson};
use crate::town::TownPlugin;
use crate::ui::UiPlugin;
use crate::bob::BobPlugin;

use bevy::pbr::VolumetricLight;
use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;
use bevy::app::AppExit;
use bevy::window::{CursorGrabMode, PrimaryWindow};

use rand::prelude::*;

use std::f32::consts::*;
use std::ops::Add;

pub struct GamePlugin;

#[derive(Component)]
struct Jointy;

#[derive(Component)]
struct JointCycle;

#[derive(Component)]
pub struct Timey(pub f32);

#[derive(Component)]
struct GltfLoaded;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NimPlugin);
        app.add_plugins(PlayerPlugin);
        app.add_plugins(PersonPlugin);
        app.add_plugins(TerrainPlugin);
        app.add_plugins(TownPlugin);
        app.add_plugins(UiPlugin);
        app.add_plugins(BobPlugin);

        app.add_systems(Startup, (setup_scene, cursor_grab));
        app.add_systems(Update, (
            update_timers,
            animate_joints,
            animate_joint_cycle,
            ray_cast_system,
            exit_system
        ));

        app.add_observer(tag_gltf_heirachy);
    }
}

fn cursor_grab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = q_windows.single_mut();
    primary_window.cursor_options.grab_mode = CursorGrabMode::Confined;
    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor_options.visible = false;
}

fn ray_cast_system(
    mut ray_cast: MeshRayCast,
    cam: Query<(&Transform, &GlobalTransform), With<Player>>,
    buttons: Res<ButtonInput<MouseButton>>,
    query: Query<(), With<Pickable>>,
    mut commands: Commands,
) {
    let (transform, global_transform) = cam.single();
    let pos = transform.translation;
    let ray = Ray3d::new(Vec3::new(pos.x, pos.y + 1.5, pos.z),  global_transform.forward());

    let filter = |entity| query.contains(entity);
//    let early_exit_test = |_entity| false;
    let visibility = RayCastVisibility::Any;

    let settings = RayCastSettings::default()
        .with_filter(&filter)
        //.with_early_exit_test(&early_exit_test)
        .with_visibility(visibility);

    let hits = ray_cast.cast_ray(ray, &settings);
    if buttons.just_pressed(MouseButton::Left) && hits.len() > 0 {
        info!("{:?}", hits.len());
        for (e, rmh) in hits.iter() {
            info!("{:?}", rmh.triangle.unwrap());
            commands.trigger_targets(SpawnPerson { pos:rmh.triangle.unwrap()[0], speed: 0.0 }, *e);
            //commands.entity(*e).despawn();
            commands.entity(*e).remove::<Pickable>();
        }
    }
}

fn exit_system(
    mut exit: EventWriter<AppExit>,
    input: Res<ButtonInput<KeyCode>>
) {
    if input.pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
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
    mut ambient_light: ResMut<AmbientLight>
) {
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        VolumetricLight,
        Transform::from_xyz(0.0, 10.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-PI / 4.))
    ));

    ambient_light.brightness = 800.0;

    let mut rng = rand::thread_rng();
    let half = 60.0;
    for _ in 0..20 {
        let pos = Vec3::new(rng.gen_range(-half..half), 0.0, rng.gen_range(-half..half));
        commands.trigger(SpawnPerson { pos, speed: rng.gen_range(0.2..1.2) });
    }
}


fn tag_gltf_heirachy(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    scenes: Query<&Timey, With<SceneRoot>>,
    deets: Query<(&GlobalTransform, &Parent, Option<&Name>)>,
) {
    let root = trigger.entity();

    commands.entity(root).insert(GltfLoaded);

    let offset: f32 = match scenes.get(root) {
        Ok(timey) => timey.0,
        _ => 0.0
    };

    if let Ok(boop) = scenes.get(root) {
        println!("----{:?}, {}", boop.0, offset);
    }

    for entity in children.iter_descendants(root) {
        //info!("i: {}", entity);
        if let Ok((transform, parent, name)) = deets.get(entity) {
            if let Some(name) = name {
                //info!("n: {} {:?} {:?}", name, parent, name.starts_with("Cube"));
                if *name == Name::new("forearm") {
                    commands.entity(entity).insert((Jointy, Timey(0.0)));
                }
                if *name == Name::new("shoulder") {
                    commands.entity(entity).insert((Jointy, Timey(10.0)));
                }
                if *name == Name::new("hand") {
                    commands.entity(entity).insert((Jointy, Timey(3.0)));
                }
                if *name == Name::new("HandMesh") {
                    commands.entity(entity).insert(Pickable);
                }
                if *name == Name::new("BodyMesh") {
                    commands.entity(entity).insert(Pickable);
                }
                if *name == Name::new("HeadBone") {
                    commands.entity(entity).insert((JointCycle, Timey(3.0)));
                }
                if offset > 0.0 && *name == Name::new("LegLowerBone") {
                    info!("off: {}", offset);
                    commands.entity(entity).insert((JointCycle, Timey(offset)));
                }

            } else {
                 //info!("t: {:?}", transform);
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

fn animate_joint_cycle(
    mut joints: Query<(&mut Transform, &Timey), With<JointCycle>>,
) {
    for (mut t, timey) in joints.iter_mut() {
        let sec = timey.0 * 5.5;
        t.rotation =
            Quat::from_rotation_x(FRAC_PI_2 * sec.sin() * 0.5)
            .normalize() * 1.0;
    }
}
