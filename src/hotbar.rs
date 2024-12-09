use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use crate::inventory::{Inventory,ItemId};
use crate::player::Player;

#[derive(Component)]
pub struct HotbarSelected(pub u32);

#[derive(Component)]
pub struct SlotId(pub u32);

#[derive(Debug, Event)]
pub struct HotbarChangeSelected {
    pub slot_id: u32
}

pub struct HotbarPlugin;

impl Plugin for HotbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, scroll_hotbar);
        app.add_observer(hotbar_change_selected);
    }
}

fn setup(
    mut commands: Commands,
){

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


fn scroll_hotbar(
    mut evr_scroll: EventReader<MouseWheel>,
    mut hotbar: Query<&mut HotbarSelected>,
    mut commands: Commands
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
        commands.trigger(HotbarChangeSelected { slot_id: next });
    }

}

fn hotbar_change_selected(
    trigger: Trigger<HotbarChangeSelected>,
    mut slots: Query<(&SlotId, &mut Text, &mut BackgroundColor)>,
    inv: Query<&Inventory, With<Player>>,
) {
    let selected = trigger.event().slot_id;
    let inv_player = inv.single();

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
