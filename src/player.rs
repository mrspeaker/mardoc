use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::pbr::NotShadowCaster;
use std::f32::consts::*;
use crate::inventory::{Inventory,ItemStack,ItemId};
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MainCamera;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (move_player_pos, move_player_view));
    }
}

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

    let mut inv = Inventory::new();
    inv.add_item(ItemStack::test());
    inv.add_item(ItemStack {
        item_id: ItemId::Head,
        item_type: ItemId::Head.get_default_type(),
        num: 1
    });
    inv.add_item(ItemStack {
        item_id: ItemId::Apple,
        item_type: ItemId::Apple.get_default_type(),
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
                SceneRoot(
                    asset_server
                        .load(GltfAssetLabel::Scene(0).from_asset("hand.glb"))),
                Transform::from_xyz(0.7, 0.8, -1.55)
                    .with_rotation(Quat::from_rotation_y(PI / 1.))
                    .with_scale(Vec3::splat(3.0)),
                NotShadowCaster

            ));

    });


}

fn move_player_view(
    mut mouse_motion: EventReader<MouseMotion>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let mut transform = player.single_mut();
    for motion in mouse_motion.read() {
        let yaw = -motion.delta.x * 0.002;
        let pitch = -motion.delta.y * 0.002;
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
    transform.translation.y = 0.0; // Force to ground
}
