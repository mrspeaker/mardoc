use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use std::f32::consts::*;
use crate::inventory::{Inventory,ItemStack,ItemId};
use crate::person::{Pickable,SpawnPerson,SpawnBodyPart};
use crate::hotbar::{HotbarSelected, HotbarChangeSelected};
use crate::terrain::Terrain;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct ToolViz;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (
            ray_cast_forward,
            ray_cast_down,
            move_player_pos,
            move_player_view
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

    let mat = MeshMaterial3d(materials.add(StandardMaterial {
        base_color: Srgba::hex("#443333").unwrap().into(),
        ..default()
    }));

    commands.spawn((
        Name::new("Cursor"),
        Mesh3d(meshes.add(Cuboid::new(0.2, 0.2, 0.2))),
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
            Transform::from_xyz(0., 1.5, 0.)
                .looking_at(Vec3::new(0., 1.5, -1.0), Vec3::Y),
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
    mut mouse_motion: EventReader<MouseMotion>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let mut transform = player.single_mut();
    for motion in mouse_motion.read() {
        let yaw = -motion.delta.x * 0.001;
        let pitch = -motion.delta.y * 0.001;
        // Order of rotations is important, see <https://gamedev.stackexchange.com/a/136175/103059>
        transform.rotate_y(yaw);
        transform.rotate_local_x(pitch);
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
    mut commands: Commands,
    mut ray_cast: MeshRayCast,
    cam: Query<(&Transform, &GlobalTransform), With<Player>>,
    buttons: Res<ButtonInput<MouseButton>>,
    query: Query<(), With<Pickable>>,
    hotbar: Query<&HotbarSelected>,
    inv: Query<&Inventory, With<Player>>,
    mut cursor: Query<&mut Transform, (With<Cursor>, Without<Player>)>
) {
    let selected = hotbar.single().0;
    let inv_player = inv.single();
    let tool = inv_player.map.get(&selected);

    let mut cursor_transform = cursor.single_mut();

    let (transform, global_transform) = cam.single();
    let pos = transform.translation;
    let ray = Ray3d::new(Vec3::new(pos.x, pos.y + 1.5, pos.z),  global_transform.forward());

    let filter = |entity| query.contains(entity);
//    let early_exit_test = |_entity| false;

    let settings = RayCastSettings::default()
        .with_filter(&filter);
        //.with_early_exit_test(&early_exit_test)

    let hits = ray_cast.cast_ray(ray, &settings);

    if hits.len() == 0 {
        cursor_transform.translation.y = -10.0;
        return;
    }

    for (e, rmh) in hits.iter() {
        cursor_transform.translation = rmh.point;//rmh.triangle.unwrap()[0];

        if buttons.just_pressed(MouseButton::Left) {
            let tool_id = tool.map(|t| t.item_id).unwrap_or(ItemId::Fist);
            info!("{:?} {:?}", hits.len(), tool_id);
            info!("{:?}", rmh.triangle.unwrap());
            if tool_id == ItemId::Cloner {
                info!("{:?}", rmh.triangle.unwrap());
                //commands.trigger_targets(SpawnPerson { pos:rmh.triangle.unwrap()[0], speed: 0.0 }, *e);
                commands.trigger_targets(SpawnPerson { pos:rmh.point, speed: 0.0, normal: rmh.normal }, *e);
                commands.entity(*e).remove::<Pickable>();
            } else if tool_id == ItemId::Sword {
                commands.entity(*e).despawn_recursive();
            } else if tool_id == ItemId::Fist {
                //
            } else if tool_id == ItemId::Head {
                // Spawn the thing.
                commands.trigger_targets(SpawnBodyPart { pos:rmh.point, item_id: ItemId::Head, normal: rmh.normal }, *e);
            } else if tool_id == ItemId::Leg {
                commands.trigger_targets(SpawnBodyPart { pos:rmh.point, item_id: ItemId::Leg, normal: rmh.normal }, *e);
            }
        }
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
        transform.translation.y = rmh.point.y+0.1;
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
