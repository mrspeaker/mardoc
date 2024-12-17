use bevy::prelude::*;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use std::f32::consts::*;

use crate::inventory::{Inventory,ItemStack,ItemId};
use crate::person::{Pickable, SpawnPerson, SpawnBodyPart, HitBodyPart};
use crate::hotbar::{HotbarSelected, HotbarChangeSelected};
use crate::terrain::Terrain;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct ToolViz;

#[derive(Resource)]
struct RaycastTarget {
    dir: Dir3,
    point: Option<Vec3>,
    normal: Vec3,
    mesh: Option<Entity>,
    mesh_point: Vec3
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (
            ray_cast_forward,
            ray_cast_down,
            move_player_pos,
            move_player_view,
            use_tool,
            cursor_ray_align
        ));
        app.add_observer(switch_tool_viz);
    }
}

#[derive(Component)]
struct Cursor;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {

    commands.insert_resource(RaycastTarget {
        dir: Dir3::Z,
        point: None,
        normal: Vec3::ZERO,
        mesh: None,
        mesh_point: Vec3::ZERO
    });

    let mat = MeshMaterial3d(materials.add(StandardMaterial {
        base_color: Srgba::hex("#443333").unwrap().into(),
        ..default()
    }));

    commands.spawn((
        Name::new("Cursor"),
        Mesh3d(meshes.add(Cuboid::new(0.02, 0.02, 0.25))),
        mat.clone(),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Cursor
    ));

    let mut inv = Inventory::new();
    inv.add_item(ItemStack {
        item_id: ItemId::Fist,
        item_type: ItemId::Fist.get_default_type(),
        num: 1
    });
    inv.add_item(ItemStack {
        item_id: ItemId::Sword,
        item_type: ItemId::Sword.get_default_type(),
        num: 1
    });
    inv.add_item(ItemStack {
        item_id: ItemId::Cloner,
        item_type: ItemId::Cloner.get_default_type(),
        num: 1
    });
    inv.add_item(ItemStack {
        item_id: ItemId::Head,
        item_type: ItemId::Head.get_default_type(),
        num: 1
    });
    inv.add_item(ItemStack {
        item_id: ItemId::Leg,
        item_type: ItemId::Leg.get_default_type(),
        num: 1
    });

    commands.spawn((
        Name::new("Player"),
        Player,
        Transform::from_xyz(0., 0., 25.0),
        Visibility::Visible,
        inv
    )).with_children(|parent| {
        parent.spawn((
            Camera3d::default(),
            Camera {
                hdr: true,
                ..default()
            },
            Transform::from_xyz(0., 1.5, 0.),
                //.looking_to(-Dir3::Z, Vec3::Y),
            MainCamera
        ));

        parent
            .spawn((
                Name::new("Hand"),
                ToolViz,
                SceneRoot(
                    asset_server
                        .load(GltfAssetLabel::Scene(0).from_asset("hand.glb"))),
                Transform::from_xyz(0.7, 0.8, -1.55)
                    .with_rotation(Quat::from_rotation_y(PI / 1.))
                    .with_scale(Vec3::splat(3.0)),
                NotShadowCaster

            ));

        parent
            .spawn((
                Name::new("Cleaver"),
                ToolViz,
                SceneRoot(
                    asset_server
                        .load(GltfAssetLabel::Scene(0).from_asset("cleaver.glb"))),
                Transform::from_xyz(0.65, 0.7, -1.75)
                    .with_rotation(Quat::from_euler(EulerRot::YXZ, -PI/2.5, 0., -PI / 2.))
                    .with_scale(Vec3::splat(2.0)),
                NotShadowCaster,
                Visibility::Hidden
            ));

        parent
            .spawn((
                Name::new("Gun"),
                ToolViz,
                SceneRoot(
                    asset_server
                        .load(GltfAssetLabel::Scene(0).from_asset("gun.glb"))),
                Transform::from_xyz(0.65, 0.6, -1.75)
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, -PI/2.5, 0.))
                    .with_scale(Vec3::splat(2.0)),
                NotShadowCaster,
                NotShadowReceiver,
                Visibility::Hidden
            ));


    });


}

fn move_player_view(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = player.get_single_mut() else {
        return;
    };
    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * 0.002;
        let delta_pitch = -delta.y * 0.001;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

fn move_player_pos(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let mut transform = player.single_mut();

    let mut sp = 1.0;
    if input.pressed(KeyCode::ShiftLeft) {
        sp *= 5.0;
    }

    let mut mo = Vec3::new(0.0, 0.0, 0.0);
    if input.pressed(KeyCode::KeyW) {
        mo += transform.local_z() * -sp;
    }
    if input.pressed(KeyCode::KeyS) {
        mo += transform.local_z() * sp;
    }
    if input.pressed(KeyCode::KeyA) {
        mo += transform.local_x() * -sp;
    }
    if input.pressed(KeyCode::KeyD) {
        mo += transform.local_x() * sp;
    }

    transform.translation += mo * time.delta_secs() * 8.0;
    //transform.translation.y = 0.0; // Force to ground
}

fn ray_cast_forward(
    mut ray_cast: MeshRayCast,
    player_query: Query<(&Transform, &GlobalTransform), With<Player>>,
    meshes_query: Query<&GlobalTransform, With<Pickable>>,
    mut ray_target: ResMut<RaycastTarget>,
) {
    let (transform, global_transform) = player_query.single();
    let pos = transform.translation;
    let ray = Ray3d::new(Vec3::new(pos.x, pos.y + 1.5, pos.z),  global_transform.forward());

    let filter = |entity| meshes_query.contains(entity);
    // let early_exit_test = |_entity| false;

    let settings = RayCastSettings::default()
        .with_filter(&filter);
        //.with_early_exit_test(&early_exit_test)

    ray_target.dir = ray.direction;
    let hits = ray_cast.cast_ray(ray, &settings);

    if hits.len() == 0 {
        ray_target.point = None;
        ray_target.mesh = None;
        return;
    }

    for (e, rmh) in hits.iter() {
        let world_pos = rmh.point;
        let normal = rmh.normal;
        // Hit position to local space
        let mesh_local_pos = meshes_query.get(*e).unwrap().affine().inverse().transform_point3(world_pos);

        ray_target.point = Some(world_pos);
        ray_target.normal = normal;
        ray_target.mesh = Some(*e);
        ray_target.mesh_point = mesh_local_pos;
    }
}

fn ray_cast_down(
    mut ray_cast: MeshRayCast,
    mut player: Query<(&mut Transform, &GlobalTransform), With<Player>>,
    query: Query<(), With<Terrain>>,
) {
    let (mut transform, global_transform) = player.single_mut();
    let pos = transform.translation;
    let ray = Ray3d::new(
        Vec3::new(pos.x, pos.y+0.8, pos.z),
        global_transform.down()
    );

    let filter = |entity| query.contains(entity);
    let settings = RayCastSettings::default()
        .with_filter(&filter);

    let hits = ray_cast.cast_ray(ray, &settings);
    if hits.len() == 0 {
        transform.translation.y += 0.1;
        return;
    }
    for (_e, rmh) in hits.iter() {
        if rmh.distance < 1.5 {
            transform.translation.y = rmh.point.y+0.1;
        } else {
            transform.translation.y -= 0.1;
        }
    }
}


fn switch_tool_viz(
    trigger: Trigger<HotbarChangeSelected>,
    mut tools: Query<(&mut Visibility, &Name), With<ToolViz>>
) {
    let next = trigger.event().slot_id;
    for (mut vis, name) in tools.iter_mut() {
        *vis = Visibility::Hidden;
        if name.starts_with("Hand") && next != 1 && next != 2 {
            *vis = Visibility::Visible;
        }
        if name.starts_with("Cleaver") && next == 1 {
            *vis = Visibility::Visible;
        }
        if name.starts_with("Gun") && next == 2 {
            *vis = Visibility::Visible;
        }
    }
}

fn cursor_ray_align(
    ray_target: ResMut<RaycastTarget>,
    mut cursor: Query<&mut Transform, With<Cursor>>
) {
    let mut cursor_transform = cursor.single_mut();
    if let Some(p) = ray_target.point {
        cursor_transform.translation = p;
        let fwd = cursor_transform.forward();
        cursor_transform.translation += fwd * 0.125;
        cursor_transform.look_to(ray_target.normal, Dir3::Y);
    } else {
        cursor_transform.translation.y = -10.0;
    }
}


fn use_tool(
    ray_target: ResMut<RaycastTarget>,
    hotbar: Query<&HotbarSelected>,
    buttons: Res<ButtonInput<MouseButton>>,
    inv: Query<&Inventory, With<Player>>,
    mut commands: Commands

) {
    let Some(_point) = ray_target.point else {
        return;
    };
    let Some(mesh) = ray_target.mesh else {
        return;
    };

    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let selected = hotbar.single().0;
    let inv_player = inv.single();
    let tool = inv_player.map.get(&selected);
    let tool_id = tool.map(|t| t.item_id).unwrap_or(ItemId::Fist);

    let normal = ray_target.normal;
    let mesh_point = ray_target.mesh_point;

    if tool_id == ItemId::Cloner {
        commands.trigger_targets(
            SpawnPerson { pos: mesh_point, speed: 0.0, normal },
            mesh
        );
        return;
    }

    if tool_id == ItemId::Sword {
        commands.entity(mesh).remove_parent();
        commands.entity(mesh).despawn_recursive();
    } else if tool_id == ItemId::Fist {
        commands.trigger_targets(
            HitBodyPart { item_id: ItemId::Apple, dir: ray_target.dir, power: 20.0 },
            mesh
        );
    } else if tool_id == ItemId::Head {
        // Spawn the thing.
        commands.trigger_targets(
            SpawnBodyPart { pos: mesh_point, item_id: ItemId::Head, normal },
            mesh
        );
    } else if tool_id == ItemId::Leg {
        commands.trigger_targets(
            SpawnBodyPart { pos: mesh_point, item_id: ItemId::Leg, normal },
            mesh
        );
    }

}
