use bevy::prelude::*;
use crate::player::Player;
use crate::inventory::Inventory;

#[derive(Component)]
struct Ui;

#[derive(Component)]
struct SlotId(u32);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, update_slot_ui);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>) {

    let mesh = meshes.add(Sphere::default().mesh().uv(12, 8));

    commands.spawn((
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
        },
        BackgroundColor(Srgba::hex("#ff0000").unwrap().into()),
    ));
    commands.spawn((
        Node {
            width: Val::Px(4.0),
            height: Val::Px(4.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            margin: UiRect {
                left: Val::Px(-5.0), // Offset to center
                top: Val::Px(-3.0),
                ..default()
            },
            ..default()
        },
        BackgroundColor(Srgba::hex("#000000").unwrap().into()),
    ));

    for i in 0..3 {
        commands.spawn((
            Name::new("slot0"),
            SlotId(i),
            Node {
                width: Val::Px(45.0),
                height: Val::Px(45.0),
                position_type: PositionType::Absolute,
                left: Val::Px(50.0 * i as f32 + 50.0),
                top: Val::Percent(90.0),
                margin: UiRect {
                    left: Val::Px(-5.0), // Offset to center
                    top: Val::Px(-5.0),
                    ..default()
                },
                ..default()
            },
            Text::new("."),
            BackgroundColor(Srgba::hex("#555555").unwrap().into())
        ));

    }



}


fn update_slot_ui(
    mut query: Query<(&SlotId, &mut Text)>,
    inv: Query<&Inventory, With<Player>>

) {
    let inv_player = inv.single();
    for (slot, mut text) in query.iter_mut() {
        **text = format!("n:{}", match inv_player.map.get(&slot.0) {
            Some(&s) => s.num,
            _ => 0
        });
    }

}
