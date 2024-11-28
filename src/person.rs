use std::f32::consts::*;

use bevy::prelude::*;

pub struct PersonPlugin;

#[derive(Debug, Event)]
pub struct SpawnPerson(pub Vec3);

#[derive(Component)]
struct Person;

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

    let mat = MeshMaterial3d(materials.add(StandardMaterial {
        base_color: Srgba::hex("#ffffff").unwrap().into(),
        ..default()
    }));

    commands.spawn((
        Name::new("Person"),
        Transform::from_translation(event.0),
        Person
    )).with_children(|parent| {
        parent.spawn((
            Name::new("Body"),
            Mesh3d(meshes.add(Cuboid::new(3.0, 8.0, 1.5))),
            mat.clone(),
            Transform::from_xyz(0.0, 4.0, 0.0)
        ));

        parent
            .spawn((
                Name::new("Arm1"),
                SceneRoot(
                    asset_server
                        .load(GltfAssetLabel::Scene(0).from_asset("arm.glb"))),
                Transform::from_xyz(-1.4, 4.0, 0.0)
                    .with_rotation(Quat::from_rotation_z(PI / 2.))
                    .with_scale(Vec3::new(10.0, 10.0, 10.0))

            ));

        parent
            .spawn((
                Name::new("Arm2"),
                SceneRoot(
                    asset_server
                        .load(GltfAssetLabel::Scene(0).from_asset("arm.glb"))),
                Transform::from_xyz(1.4, 4.0, 0.0)
                    .with_rotation(Quat::from_euler(EulerRot::YXZ, 0.0, 0., -PI / 2.))
                    .with_scale(Vec3::new(10.0, 10.0, 10.0))
            ));
    });

}

fn move_person(
    time: Res<Time>,
    mut q: Query<&mut Transform, With<Person>>
) {
    for mut transform in q.iter_mut() {
        transform.translation.x += time.elapsed_secs().sin() * 0.1;
    }
}
