use std::f32::consts::*;

use bevy::prelude::*;

pub struct PersonPlugin;

#[derive(Debug, Event)]
pub struct SpawnPerson {
    pub pos: Vec3,
    pub speed: f32
}

#[derive(Component)]
struct Person;

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

    let mat = MeshMaterial3d(materials.add(StandardMaterial {
        base_color: Srgba::hex("#ff00ff").unwrap().into(),
        ..default()
    }));

    commands.spawn((
        Name::new("Person"),
        Transform::from_translation(event.pos),
        Visibility::Visible,
        Person,
        Speed(event.speed)
    )).with_children(|parent| {

        let h = 1.8;
        let w = 0.8;

        parent.spawn((
            Name::new("Body"),
            Mesh3d(meshes.add(Cuboid::new(w, h, 0.5))),
            mat.clone(),
            Transform::from_xyz(0.0, h/2.0, 0.0)
        ));

        parent
            .spawn((
                Name::new("Arm1"),
                SceneRoot(
                    asset_server
                        .load(GltfAssetLabel::Scene(0).from_asset("arm.glb"))),
                Transform::from_xyz(-w/2.0, h * 0.75, 0.0)
                    .with_rotation(Quat::from_rotation_z(PI / 2.))
                    .with_scale(Vec3::splat(2.0))

            ));

        parent
            .spawn((
                Name::new("Arm2"),
                SceneRoot(
                    asset_server
                        .load(GltfAssetLabel::Scene(0).from_asset("arm.glb"))),
                Transform::from_xyz(w/2.0, h*0.75, 0.0)
                    .with_rotation(Quat::from_euler(EulerRot::YXZ, 0.0, 0., -PI / 2.))
                    .with_scale(Vec3::splat(2.0))
            ));
    });

}

fn move_person(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &Speed), With<Person>>
) {
    let dt = time.delta_secs();
    for (mut transform, speed) in q.iter_mut() {
        transform.rotate_local_y(speed.0 * 0.5 * dt);
        let move_amount = transform.local_z() * speed.0 * dt;
        transform.translation += move_amount;
    }
}
