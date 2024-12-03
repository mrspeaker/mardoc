use std::f32::consts::*;

use bevy::prelude::*;
use crate::game::Timey;
use crate::bob::Bob;

pub struct PersonPlugin;

#[derive(Debug, Event)]
pub struct SpawnPerson {
    pub pos: Vec3,
    pub speed: f32
}

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Pickable;

#[derive(Component)]
struct Speed(f32);

impl Plugin for PersonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_person);
        app.add_observer(spawn_person);
    }
}

fn spawn_person(
    trigger: Trigger<SpawnPerson>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let event = trigger.event();

    let id = trigger.entity();

    let posp = if let Some(_e) = commands.get_entity(id) {
        Vec3::new(0.0, 0.0, 0.0)
    } else {
       event.pos
    };

    let mat = MeshMaterial3d(materials.add(StandardMaterial {
        base_color: Srgba::hex("#FF9E78").unwrap().into(),
        ..default()
    }));

    let perp = commands.spawn((
        Name::new("Person"),
        Transform::from_translation(posp),//event.pos),
        Visibility::Visible,
        Person,
        Bob(0.0),
        Speed(event.speed)
    )).with_children(|parent| {

        let h = 1.6;
        let w = 0.75;

        /*parent.spawn((
            Name::new("Body"),
            Mesh3d(meshes.add(Cuboid::new(w, h, 0.5))),
            mat.clone(),
            Transform::from_xyz(0.0, h/2.0, 0.0),
            Pickable
        ));*/

        parent
            .spawn((
                Name::new("BodyOdy"),
                SceneRoot(
                    asset_server
                        .load(GltfAssetLabel::Scene(0).from_asset("body.glb"))),
                Transform::from_xyz(0.0, h * 0.4, 0.0)
                    //.with_rotation(Quat::from_rotation_z(PI / 2.))
                    .with_scale(Vec3::splat(1.0))

            )).with_children(|body_parent| {
                body_parent
                    .spawn((
                        Name::new("Arm1"),
                        SceneRoot(
                            asset_server
                                .load(GltfAssetLabel::Scene(0).from_asset("arm.glb"))),
                        Transform::from_xyz(-0.1, 0.6, 0.0)
                            .with_rotation(Quat::from_rotation_z(PI / 2.))
                            .with_scale(Vec3::splat(1.0))

                    ));

                body_parent
                    .spawn((
                        Name::new("Arm2"),
                        SceneRoot(
                            asset_server
                                .load(GltfAssetLabel::Scene(0).from_asset("arm.glb"))),
                        Transform::from_xyz(0.1, 0.6, 0.0)
                            .with_rotation(Quat::from_euler(EulerRot::YXZ, 0.0, 0., -PI / 2.))
                            .with_scale(Vec3::splat(1.0))
                    ));

                body_parent
                    .spawn((
                        Name::new("head"),
                        SceneRoot(
                            asset_server
                                .load(GltfAssetLabel::Scene(0).from_asset("head.glb"))),
                        Transform::from_xyz(0.0, 0.71, -0.02)
                        //s.with_rotation(Quat::from_rotation_z(PI / 2.))
                        //.with_scale(Vec3::splat(1.5))
                    ));



                body_parent
                    .spawn((
                        Name::new("leg1"),
                        Timey(0.9),
                        SceneRoot(
                            asset_server
                                .load(GltfAssetLabel::Scene(0).from_asset("leg.glb"))),
                        Transform::from_xyz(-0.1, h * 0.1, 0.0)
                        //.with_rotation(Quat::from_rotation_x(PI / 2.))
                            .with_scale(Vec3::splat(1.0))

                    ));

                body_parent
                    .spawn((
                        Name::new("leg2"),
                        Timey(9.5),
                        SceneRoot(
                            asset_server
                                .load(GltfAssetLabel::Scene(0).from_asset("leg.glb"))),
                        Transform::from_xyz(0.1, h * 0.1, 0.0)
                        //.with_rotation(Quat::from_rotation_x(-PI / 2.))
                            .with_scale(Vec3::splat(1.0))

                    ));


            });

    }).id();

    if let Some(_e) = commands.get_entity(id) {
        //commands.entity(e);//.entity.push_children((perp));
        commands.entity(id).add_child(perp);
    }

}

fn move_person(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &Speed), With<Person>>
) {
    let dt = time.delta_secs();
    for (mut transform, speed) in q.iter_mut() {
        transform.rotate_y(speed.0 * 0.5 * dt);
        let move_amount = transform.forward() * speed.0 * dt;
        transform.translation += move_amount;
        transform.rotation = transform.rotation.normalize();
    }
}
