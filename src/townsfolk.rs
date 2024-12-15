use bevy::prelude::*;

pub struct TownsfolkPlugin;

#[derive(Component)]
pub struct LookingForWork;

/// Task for a townsfolk
pub enum TownsfolkTaskType {
    Idle,
    Wandering(Vec3),
    Fleaing,
}

#[derive(Component)]
pub struct TownsfolkTask {
    pub task: TownsfolkTaskType
}

impl Plugin for TownsfolkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, schedule_task);
    }
}

fn schedule_task(
    mut commands: Commands,
    lfw_q: Query<Entity, With<LookingForWork>>
){
    for e in lfw_q.iter() {
        info!("What is my purpose?");
        commands.entity(e).remove::<LookingForWork>();
        commands.entity(e).insert(TownsfolkTask { task: TownsfolkTaskType::Idle });
    }
}


fn init_task(
    mut commands: Commands,
    q: Query<Entity, Added<TownsfolkTask>>
){
    //
}
