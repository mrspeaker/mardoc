use bevy::prelude::*;
use std::ops::Add;
use std::f32::consts::*;

use crate::game::Timey;
use crate::bob::Bob;
use crate::inventory::ItemId;

pub struct PersonPlugin;

#[derive(Debug, Event)]
pub struct SpawnPerson {
    pub pos: Vec3,
    pub speed: f32,
    pub normal: Vec3
}

#[derive(Debug, Event)]
pub struct SpawnBodyPart {
    pub pos: Vec3,
    pub item_id: ItemId,
    pub normal: Vec3
}

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Jointy;

#[derive(Component)]
pub struct JointCycle;

#[derive(Component)]
pub struct Pickable;

#[derive(Component)]
struct Speed(f32);

impl Plugin for PersonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            move_person,
            animate_joints,
            animate_joint_cycle
        ));
        app.add_observer(spawn_person);
        app.add_observer(spawn_bodypart);
    }
}

fn spawn_person(
    trigger: Trigger<SpawnPerson>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let event = trigger.event();
    let normal = event.normal;

    let id = trigger.entity();

    let posp = if let Some(_e) = commands.get_entity(id) {
       // Vec3::new(0.0, 0.0, 0.0)
       event.pos
    } else {
       event.pos
    };

    let perp = commands.spawn((
        Name::new("Person"),
        Transform::from_translation(posp)
            .looking_to(normal, Dir3::Y),
        Visibility::Visible,
        Person,
        Bob(0.0),
        Speed(event.speed)
    )).with_children(|parent| {

        let h = 1.6;

        parent
            .spawn((
                Name::new("BodyOdy"),
                SceneRoot(
                    asset_server
                        .load(GltfAssetLabel::Scene(0).from_asset("body.glb"))),
                Transform::from_xyz(0.0, h * 0.4, 0.0)
                    //.with_scale(Vec3::splat(1.3))

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
                        Transform::from_xyz(0.0, 0.71, -0.01)
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
        commands.entity(id).add_child(perp);
    }

}


fn spawn_bodypart(
    trigger: Trigger<SpawnBodyPart>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let event = trigger.event();
    let id = trigger.entity();
    let pos = event.pos;
    let item_id = event.item_id;
    let normal = event.normal;

    let perp = match item_id {
        ItemId::Head => {
            commands
                .spawn((
                    Name::new("head"),
                    Visibility::Visible,
                    SceneRoot(
                        asset_server
                            .load(GltfAssetLabel::Scene(0).from_asset("head.glb"))),
                    Transform::from_translation(pos)
                        .looking_to(normal, Dir3::Y)
                        //.with_rotation(Quat::from_euler(EulerRot::XYZ, normal.x, normal.y, normal.z))

                )).id()
        }
        _ => {
            commands
                .spawn((
                    Name::new("leg1"),
                    Timey(0.9),
                    Visibility::Visible,
                    SceneRoot(
                        asset_server
                            .load(GltfAssetLabel::Scene(0).from_asset("leg.glb"))),
                    Transform::from_translation(pos)
                        .looking_to(normal, Dir3::Y)
                        //.with_rotation(Quat::from_euler(EulerRot::XYZ, normal.x, normal.y, normal.z))

                )).id()
        }
    };

    if let Some(_e) = commands.get_entity(id) {
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

fn animate_joints(
    mut joints: Query<(&mut Transform, &Timey), With<Jointy>>,
) {
    for (mut t, timey) in joints.iter_mut() {
        let sec = timey.0;
        t.rotation =
            Quat::from_euler(EulerRot::XYZ,
                             0.0,
                             FRAC_PI_2 * sec.sin() * 0.5,
                             FRAC_PI_2 * sec.cos() * 0.4);
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
