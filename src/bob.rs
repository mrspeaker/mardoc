use bevy::prelude::*;

pub struct BobPlugin;

#[derive(Component)]
pub struct Bob(pub f32);

impl Plugin for BobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bob_system);
    }
}

fn bob_system(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &mut Bob)>
) {
    for (mut t, mut bob) in q.iter_mut() {
        bob.0 += time.delta_secs();
        t.translation.y += (bob.0 * 10.0).sin()*0.004;
    }
}
