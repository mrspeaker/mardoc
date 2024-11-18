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
        Mesh3d(mesh.clone()),
        MeshMaterial3d(materials.add(Color::BLACK)),
        Nim
    )).with_children(|parent| {
        parent.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(materials.add(Color::hsl(0.0, 0.5, 0.5))),
            Transform::from_xyz(0.0, 0.0, 1.5)
        ));
    });

    // meshes.add(Cylinder::default())
}

fn move_nim(mut query: Query<&mut Transform, With<Nim>>, time: Res<Time>) {
    for mut t  in &mut query {
        t.rotate_y(time.delta_secs() / 2.0);
        let up = t.up();
        t.translation += up * time.delta_secs();
        if t.translation.y > 30.0 {
            t.translation.y = -1.0;
        }
    }
}
