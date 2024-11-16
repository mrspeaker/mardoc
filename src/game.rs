use crate::terrain::TerrainPlugin;
use crate::nim::NimPlugin;

use bevy::{prelude::*, render::mesh::skinning::SkinnedMesh};
use bevy::scene::SceneInstanceReady;

use std::f32::consts::*;

use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};

pub struct GamePlugin;

#[derive(Component)]
struct GltfSceneInit;

#[derive(Component)]
struct Jointy;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanOrbitCameraPlugin);
        app.add_plugins(NimPlugin);
        app.add_plugins(TerrainPlugin);
        app.add_systems(Startup, setup_scene);
        app.add_systems(Update, (tag_scene_entities, animate_joints));
    }
}

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 20., 75.)
                .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default()
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

    /*let sc = asset_server
        .load(GltfAssetLabel::Scene(0).from_asset("SimpleSkin.gltf"));
     */
    /*
    commands.spawn(SceneBundle {
        scene: asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("SimpleSkin.gltf")),
        ..default()
    });*/


    commands
        .spawn((SceneBundle {
            scene: asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("arm.gltf")),
            transform: Transform {
                translation: Vec3::new(0.0, 1.0, 0.0),
                scale:Vec3::new( 10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        }
                , GltfSceneInit))
        .observe(daf);


    commands
        .spawn((SceneBundle {
            scene: asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("arm.gltf")),
            transform: Transform {
                translation: Vec3::new(20.0, 1.0, 0.0),
                scale:Vec3::new( 10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        }
                , GltfSceneInit));

}

fn daf(
    trigger: Trigger<SceneInstanceReady>,
    children: Query<&Children>,
) {
    let entity = children.get(trigger.entity()).unwrap()[0];
    info!("trigs? {}", entity);
}

fn tag_scene_entities(
    mut commands: Commands,
    children: Query<&Children>,
    scene: Query<Entity, With<GltfSceneInit>>,
    mut deets: Query<&Name>,
) {
    for scene_entity in &scene {
        let mut loaded = false;
        info!("Is gltf scene really loaded?");
        for entity in children.iter_descendants(scene_entity) {
            if let Ok(name) = deets.get_mut(entity) {
                if *name == Name::new("forearm") {
                    commands.entity(entity).insert(Jointy);
                    loaded = true;
                }
            }
        }
        if loaded {
            commands.entity(scene_entity).remove::<GltfSceneInit>();
        }
    }
}

fn animate_joints(
    time: Res<Time>,
    mut joints: Query<&mut Transform, With<Jointy>>,
) {
    for mut t in joints.iter_mut() {
        t.rotation =
            Quat::from_rotation_z(FRAC_PI_2 * time.elapsed_seconds().sin() * 0.5);

    }
}

fn joint_animation_(
    time: Res<Time>,
    parent_query: Query<(&Parent, &Name), With<SkinnedMesh>>,
    children_query: Query<&Children>,
    mut transform_query: Query<(&mut Transform, &Name)>,
) {
    for (skinned_mesh_parent, name) in &parent_query {
        let mesh_node_entity = skinned_mesh_parent.get();
        info!("{} {}", mesh_node_entity, name);
        let mesh_node_children = children_query.get(mesh_node_entity).unwrap();
        if mesh_node_children.len() == 1 {
            //let mut t = transform_query.get_mut(mesh_node_entity).unwrap();
            //t.translation.x += 0.01;
            let first_joint_entity = mesh_node_children[0];

            let (mut first_joint_transform, name) = transform_query.get_mut(first_joint_entity).unwrap();
            info!("{}", name);
            //let first_joint_children = children_query.get(first_joint_entity).unwrap();
            //first_joint_transform.translation.x += 0.01;
            //info!("{:#?} {}", first_joint_transform.rotation, name);

            //nfirst_joint_transform.rotation =
            //    Quat::from_rotation_z(FRAC_PI_2 * time.elapsed_seconds().sin() * 2.0);
            return;
        }
/*        let first_joint_entity = mesh_node_children[1];
        let first_joint_children = children_query.get(first_joint_entity).unwrap();

        // Second joint is the first child of the first joint.
        let second_joint_entity = first_joint_children[0];
        let mut second_joint_transform = transform_query.get_mut(second_joint_entity).unwrap();

        second_joint_transform.rotation =
            Quat::from_rotation_z(FRAC_PI_2 * time.elapsed_seconds().sin() * 5.0);*/
    }
}
