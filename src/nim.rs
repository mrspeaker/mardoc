use bevy::prelude::*;

#[derive(Component)]
struct Nim;

pub struct NimPlugin;

impl Plugin for NimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, move_nim);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>) {

    let mesh = meshes.add(Sphere::default().mesh().uv(12, 8));
    commands.spawn((
        PbrBundle  {
            mesh: mesh.clone(),
            material: materials.add(Color::BLACK),
            ..default()
        },
        Nim
    )).with_children(|parent| {
        parent.spawn(
            PbrBundle {
                mesh,
                material: materials.add(Color::hsl(0.0, 0.5, 0.5)),
                transform: Transform::from_xyz(0.0, 0.0, 1.5),
                ..default()
            }
        );
    });

    // meshes.add(Cylinder::default())
}

fn move_nim(mut query: Query<&mut Transform, With<Nim>>, time: Res<Time>) {
    for mut t  in &mut query {
        t.rotate_y(time.delta_seconds() / 2.0);
        let up = t.up();
        t.translation += up * time.delta_seconds();
        if t.translation.y > 30.0 {
            t.translation.y = -1.0;
        }
    }
}
