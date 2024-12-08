use bevy::prelude::*;
use crate::player::{Player,ToolViz};
use crate::inventory::{Inventory,ItemId};
use bevy::input::mouse::MouseWheel;

#[derive(Component)]
struct Ui;

#[derive(Component)]
struct SlotId(u32);

#[derive(Component)]
pub struct HotbarSelected(pub u32);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (update_slot_ui, scroll_hotbar));
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

    commands.spawn((
        Name::new("Hotbar"),
        HotbarSelected(0),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(100.0),
            margin: UiRect {
                left: Val::Px(-150.0), // Offset to center
                top: Val::Px(-50.0),
                ..default()
            },
            ..default()
        }
    )).with_children(|p| {

        for i in 0..5 {
            p.spawn((
                Name::new("slot0"),
                SlotId(i),
                Node {
                    width: Val::Px(45.0),
                    height: Val::Px(45.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(50.0 * i as f32),
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
    });



}


fn update_slot_ui(
    mut slots: Query<(&SlotId, &mut Text, &mut BackgroundColor)>,
    inv: Query<&Inventory, With<Player>>,
    hotbar: Query<&HotbarSelected>

) {
    let inv_player = inv.single();
    let selected = hotbar.single().0;

    for (slot, mut text, mut bg) in slots.iter_mut() {
        let sm = inv_player.map.get(&slot.0);
        **text = format!("{:?} {}", sm.map(|s| s.item_id).unwrap_or(ItemId::Fist), match sm {
            Some(&s) => s.num,
            _=> 0
        });
        bg.0 = Srgba::hex("#555555").unwrap().into();
        if slot.0 == selected {
            bg.0 = Color::BLACK;
        }
    }

}

fn scroll_hotbar(
    mut evr_scroll: EventReader<MouseWheel>,
    mut hotbar: Query<&mut HotbarSelected>,
    mut tools: Query<(&mut Visibility, &Name), With<ToolViz>>
) {
    let mut selected = hotbar.single_mut();

    use bevy::input::mouse::MouseScrollUnit;
    let mut yo = 0;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                yo = (-1.0 * ev.y.signum()) as i32;
            }
            MouseScrollUnit::Pixel => {
                yo = (-1.0 * ev.y.signum()) as i32;
            }
        }
    }

    if yo == 0 { return };
    let cur = selected.0;
    let mut next = cur;
    if yo > 0 {
        if cur < 4 {
            next = cur + 1;
        } else {
            next = 0;
        }
    } else if yo < 0 {
        if cur > 0 {
            next = cur - 1;
        } else {
            next = 4;
        }
    }
    selected.0 = next;
    if next != cur {
        // switch tool viz
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

}
