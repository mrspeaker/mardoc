use bevy::prelude::*;
use std::f32::consts::*;

use crate::game::Timey;
use crate::bob::Bob;
use crate::inventory::ItemId;
use crate::townsfolk::LookingForWork;

pub struct PersonPlugin;

#[derive(Debug, Component)]
pub struct Health(pub f32);

#[derive(Debug, Event)]
pub struct SpawnPerson {
    pub pos: Vec3,
    pub speed: f32,
    pub normal: Vec3
}

#[derive(Debug, Event)]
pub struct KillPerson;

#[derive(Debug, Event)]
pub struct SpawnBodyPart {
    pub pos: Vec3,
    pub item_id: ItemId,
    pub normal: Vec3
}

#[derive(Component)]
pub struct GltfBodyPart;

#[derive(Debug, Event)]
pub struct HitBodyPart {
    pub item_id: ItemId,
    pub dir: Dir3,
    pub power: f32
}

#[derive(Component)]
pub struct Knockback {
    pub dir: Vec3,
    pub duration: Timer
}

#[derive(Component)]
pub struct BodyRoot;


#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Jointy;

#[derive(Component)]
pub struct JointCycle;

#[derive(Component)]
pub struct Pickable;

#[derive(Component)]
pub struct Carryable;

#[derive(Component)]
struct Speed(f32);

impl Plugin for PersonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            move_person,
            animate_joints,
            animate_joint_cycle,
            apply_knockback
        ));
        app.add_observer(spawn_person);
        app.add_observer(kill_person);
        app.add_observer(spawn_bodypart);
        app.add_observer(hit_bodypart);
    }
}

fn spawn_person(
    trigger: Trigger<SpawnPerson>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let event = trigger.event();
    let normal = event.normal;
    let speed = event.speed;
    let posp = event.pos;
    let id = trigger.entity();

    let perp = commands.spawn((
        Name::new("Person"),
        Transform::from_translation(posp)
            .looking_to(normal, Dir3::Y),
        Visibility::Visible,
        Person,
        BodyRoot,
        Health(100.0),
        Bob(0.0),
        LookingForWork,
        Speed(speed)
    )).with_children(|parent| {

        let h = 1.6;

        parent
            .spawn((
                Name::new("BodyOdy"),
                GltfBodyPart,
                SceneRoot(
                    asset_server
                        .load(GltfAssetLabel::Scene(0).from_asset("body.glb"))),
                Transform::from_xyz(0.0, h * 0.4, 0.0)
                    //.with_scale(Vec3::splat(1.3))

            )).with_children(|body_parent| {
                body_parent
                    .spawn((
                        Name::new("Arm1"),
                        GltfBodyPart,
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
                        GltfBodyPart,
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
                        GltfBodyPart,
                        SceneRoot(
                            asset_server
                                .load(GltfAssetLabel::Scene(0).from_asset(if speed < 0.3 { "head.glb" } else { "serhead.glb"}))),
                        Transform::from_xyz(0.0, 0.71, -0.01)
                    ));

                body_parent
                    .spawn((
                        Name::new("leg1"),
                        GltfBodyPart,
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
                        GltfBodyPart,
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
    query: Query<&GlobalTransform, With<Parent>>,
) {
    let event = trigger.event();
    let id = trigger.entity();
    let pos = event.pos;
    let item_id = event.item_id;
    let normal = event.normal;
    //let look_t = Transform::IDENTITY.look_to(normal, Dir3::Y);
    let q = Quat::from_euler(EulerRot::XYZ, normal.x, normal.y, normal.z);

    let rotation = match query.get(id) {
        Ok(t) => t.rotation() + q,
        _ => q
    };

    info!("parrr {:?}", rotation);

    let perp = match item_id {
        ItemId::Head => {
            commands
                .spawn((
                    Name::new("serhead"),
                    Visibility::Visible,
                    GltfBodyPart,
                    BodyRoot,
                    SceneRoot(
                        asset_server
                            .load(GltfAssetLabel::Scene(0).from_asset("serhead.glb"))),
                    Transform::from_translation(pos)
                        .with_rotation(rotation)
                        //.looking_to(normal, Dir3::Y)
                        .with_scale(Vec3::splat(1.5))
                        //.with_rotation(Quat::from_euler(EulerRot::XYZ, normal.x, normal.y, normal.z))

                )).id()
        },
        ItemId::Leg => {
            commands
                .spawn((
                    Name::new("leg1"),
                    Timey(0.9),
                    Visibility::Visible,
                    BodyRoot,
                    SceneRoot(
                        asset_server
                            .load(GltfAssetLabel::Scene(0).from_asset("leg.glb"))),
                    Transform::from_translation(pos)
                        .looking_to(normal, Dir3::Y)
                        //.with_rotation(Quat::from_euler(EulerRot::XYZ, normal.x, normal.y, normal.z))

                )).id()
        },
        _ => {
            commands
                .spawn((
                    Name::new("apple"),
                    Timey(0.9),
                    Visibility::Visible,
                    BodyRoot,
                    SceneRoot(
                        asset_server
                            .load(GltfAssetLabel::Scene(0).from_asset("plinth.glb"))),
                    Transform::from_translation(pos)
                        .looking_to(normal, Dir3::Y)
                        //.with_rotation(Quat::from_euler(EulerRot::XYZ, normal.x, normal.y, normal.z))

                )).id()
        }
    };

    if let Some(_e) = commands.get_entity(id) {
        commands.entity(perp).remove::<BodyRoot>();
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

fn apply_knockback(
    mut q: Query<(&mut Knockback, &mut Transform)>,
    time: Res<Time>
){
    for (mut knock, mut t) in q.iter_mut() {
        knock.duration.tick(time.delta());
        if !knock.duration.finished() {
            t.translation += knock.dir * time.delta_secs();
        }
    }
}

fn hit_bodypart(
    trigger: Trigger<HitBodyPart>,
    parent_q: Query<&Parent>,
    mut persons: Query<&mut Health, With<Person>>,
    mut commands: Commands,
) {
    let id = trigger.entity();

    let root = parent_q.root_ancestor(id);
    let Ok(mut p) = persons.get_mut(root) else {
        return;
    };
    if p.0 <= 0.0 {
        info!("{:?} ded already", p.0);
        return;
    }

    let event = trigger.event();
    let dir = event.dir;
    let power = event.power;

    p.0 -= 25.0;
    if p.0 <= 0.0 {
        commands.trigger_targets(KillPerson, root);
    } else {
        commands.entity(root).insert(Knockback {
            dir: Vec3::from(dir) * Vec3::new(1.0, 0.0, 1.0) * power,
            duration: Timer::from_seconds(0.2, TimerMode::Once)
        });
    }

}

fn kill_person(
    trigger: Trigger<KillPerson>,
    persons: Query<&GlobalTransform, With<Person>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let id = trigger.entity();

    let pos = match persons.get(id) {
        Ok(t) => t.translation(),
        _ => Vec3::ZERO
    };

    info!("You ded {:?}", pos);

    commands.entity(id).despawn_recursive();
    commands
        .spawn((
            SceneRoot(
                asset_server
                    .load(GltfAssetLabel::Scene(0).from_asset("dead.glb"))),
            Transform::from_xyz(pos.x, pos.y , pos.z),
            Carryable,
            BodyRoot,
            Person,
            Health(0.0)
        ));
}
