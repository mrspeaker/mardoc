use std::f32::consts::*;

use bevy::prelude::*;

pub struct CameraPlugin;

#[derive(Component)]
pub struct MainCamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(Update, move_person);
        //app.add_observer(spawn_person);
    }
}
