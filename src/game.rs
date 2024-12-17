use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;
use bevy::app::AppExit;
use bevy::window::{CursorGrabMode, PrimaryWindow};

use rand::prelude::*;
use std::f32::consts::*;

use crate::nim::NimPlugin;
use crate::player::PlayerPlugin;
use crate::person::{
    Pickable,
    PersonPlugin,
    SpawnPerson,
    Jointy,
    JointCycle, GltfBodyPart
};
use crate::town::TownPlugin;
use crate::townsfolk::TownsfolkPlugin;
use crate::ui::UiPlugin;
use crate::bob::BobPlugin;
use crate::hotbar::HotbarPlugin;
use crate::terrain::Terrain;

pub struct GamePlugin;

#[derive(Component)]
pub struct Timey(pub f32);

#[derive(Component)]
struct GltfLoaded;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NimPlugin);
        app.add_plugins(PlayerPlugin);
        app.add_plugins(PersonPlugin);
        app.add_plugins(TownPlugin);
        app.add_plugins(TownsfolkPlugin);
        app.add_plugins(UiPlugin);
        app.add_plugins(BobPlugin);
        app.add_plugins(HotbarPlugin);

        app.add_systems(Startup, (setup_scene, cursor_grab));
        app.add_systems(Update, (
            update_timers,
            exit_system
        ));

        app.add_observer(tag_gltf_heirachy);
    }
}

fn cursor_grab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = q_windows.single_mut();
    //primary_window.cursor_options.grab_mode = CursorGrabMode::Confined;
    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor_options.visible = false;
}

fn exit_system(
    mut exit: EventWriter<AppExit>,
    input: Res<ButtonInput<KeyCode>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {

    if input.pressed(KeyCode::Tab) {
        let mut primary_window = q_windows.single_mut();
        if primary_window.cursor_options.grab_mode == CursorGrabMode::Locked {
            primary_window.cursor_options.grab_mode = CursorGrabMode::None;
            primary_window.cursor_options.visible = true;
        } else {
            primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
            primary_window.cursor_options.visible = false;
        }
    }

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
        Transform::from_xyz(0.0, 10.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-PI / 4.))
    ));

    ambient_light.brightness = 600.0;

    let mut rng = rand::thread_rng();
    let half = 40.0;
    for _ in 0..20 {
        let pos = Vec3::new(rng.gen_range(-half..half), 0.0, rng.gen_range(-half..half));
        let speed = rng.gen_range(0.2..0.4);
        let dir = Vec3::new(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.0)).normalize();
        info!("{:?}", dir);
        commands.trigger(SpawnPerson { pos, speed, normal: dir });
    }
}

fn boop(
    mut commands: Commands,
    mut meshy: Query<Entity, Added<Mesh3d>>
){
    for e in meshy.iter_mut() {
        commands.entity(e).insert(Pickable);
    }
}

fn tag_gltf_heirachy(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    //body_parts: Query<(Option<&Timey>, Option<&Name>), (With<SceneRoot>, With<GltfBodyPart>)>,
    body_parts: Query<(Option<&Timey>, Option<&Name>), With<SceneRoot>>,
    deets: Query<(&GlobalTransform, &Parent, Option<&Name>)>,
) {
    let root = trigger.entity();

    commands.entity(root).insert(GltfLoaded);

    if let Ok(parts) = body_parts.get(root) {
        let (timey, nameo) = parts;
        info!("Scene: {:?}", nameo.map_or("-",|v|v));
        let offset: f32 = timey.map_or(0.0, |Timey(v)| *v);

        for entity in children.iter_descendants(root) {
            info!("bodyparts i: {}", entity);
            if let Ok((transform, parent, name)) = deets.get(entity) {
                let name = name.map_or("-", |v|v);
                info!("n: {} {:?}", name, parent);
                if name == "forearm" {
                    commands.entity(entity).insert((Jointy, Timey(0.0)));
                }
                if name == "shoulder" {
                    commands.entity(entity).insert((Jointy, Timey(10.0)));
                }
                if name == "hand" {
                    commands.entity(entity).insert((Jointy, Timey(3.0)));
                }
                if name.ends_with("Mesh") {
                    commands.entity(entity).insert(Pickable);
                }
                if name.ends_with("Floor") {
                    commands.entity(entity).insert(Terrain);
                }
                if name == "HeadBone" {
                    commands.entity(entity).insert((JointCycle, Timey(3.0), Pickable));
                }
                if name == "SerheadBone" {
                    commands.entity(entity).insert((Jointy, Timey(3.0), Pickable));
                }

                if offset > 0.0 && name == "LegLowerBone" {
                    commands.entity(entity).insert((JointCycle, Timey(offset)));
                }
            }

        }

    }
}

/*
fn gltf_bodypart_matcher(
    name: &str
) {
    match name {
        "forearm" => (Jointy, Timey(0.0)),
        _ if name.starts_with("Floor") => (Terrain)
    }
}
*/
