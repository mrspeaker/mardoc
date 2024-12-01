use std::f32::consts::*;

use bevy::prelude::*;

pub struct TownPlugin;

impl Plugin for TownPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    commands
        .spawn((
            Name::new("Town1"),
            SceneRoot(
                asset_server
                    .load(GltfAssetLabel::Scene(0).from_asset("building1.glb"))),
            Transform::from_xyz(0.0, 0.0, 20.0)
                .with_rotation(Quat::from_rotation_y(PI / 2.))
                .with_scale(Vec3::splat(2.0))
        ));

}
