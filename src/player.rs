use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::pbr::NotShadowCaster;

pub struct PlayerPlugin;

#[derive(Component)]
struct Player;

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
) {

    let mat = MeshMaterial3d(materials.add(StandardMaterial {
        base_color: Srgba::hex("#ff00ff").unwrap().into(),
        ..default()
    }));

    commands.spawn((
        Name::new("Player1"),
        Player,
        Transform::from_xyz(0., 0., 50.0)
    )).with_children(|parent| {
        parent.spawn((
            Camera3d::default(),
            Transform::from_xyz(0., 1.5, 0.)
                .looking_at(Vec3::new(0., 1.5, 0.), Vec3::Y),
        ));

        parent.spawn((
            Name::new("Arm"),
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.5))),
            mat,
            Transform::from_xyz(0.2, 0.8, -10.25),
            NotShadowCaster,
        ));

    });
}

fn move_player_view(
    mut mouse_motion: EventReader<MouseMotion>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let mut transform = player.single_mut();
    for motion in mouse_motion.read() {
        let yaw = -motion.delta.x * 0.003;
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

    let mut mo = Vec3::new(0.0, 0.0, 0.0);
    if input.pressed(KeyCode::KeyW) {
        mo += transform.local_z() * -1.0;
    }
    if input.pressed(KeyCode::KeyS) {
        mo += transform.local_z() * 1.0;
    }
    if input.pressed(KeyCode::KeyA) {
        mo += transform.local_x() * -1.0;
    }
    if input.pressed(KeyCode::KeyD) {
        mo += transform.local_x() * 1.0;
    }

    transform.translation += mo * time.delta_secs() * 8.0;
    transform.translation.y = 0.0; // Force to ground
}
