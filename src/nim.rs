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
            mesh,
            material: materials.add(Color::BLACK),
            ..default()
        },
        Nim
    ));
}

fn move_nim(mut query: Query<&mut Transform, With<Nim>>, time: Res<Time>) {
    for mut t  in &mut query {
        t.rotate_y(time.delta_seconds() / 2.0);
        let up = t.up();
        t.translation += up * time.delta_seconds();
    }
}
