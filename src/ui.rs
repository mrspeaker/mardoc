use bevy::prelude::*;
use crate::player::Player;

#[derive(Component)]
struct Ui;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>) {

    let mesh = meshes.add(Sphere::default().mesh().uv(12, 8));

    commands.spawn((
        Name::new("Crosshair"),
        Node {
            width: Val::Px(4.0),
            height: Val::Px(4.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            margin: UiRect {
                left: Val::Px(-5.0), // Offset to center
                top: Val::Px(-5.0),
                ..default()
            },
            ..default()
        })).with_children(|parent| {
        parent.spawn((
            Node {
                width: Val::Px(4.0),
                ..default()
            },
            BackgroundColor(Srgba::hex("#ff0000").unwrap().into()),
        ));
        parent.spawn((
            Node {
                width: Val::Px(4.0),
                ..default()
            },
            BackgroundColor(Srgba::hex("#000000").unwrap().into()),
        ));
    });

}

