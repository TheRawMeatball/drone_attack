use crate::gameplay::*;
use bevy::{
    ecs::{ScheduledOperation, SetState},
    prelude::*,
};

pub fn player_health_system(
    mut query: Query<(&mut Handle<ColorMaterial>, &Damage), (Changed<Damage>, With<Player>)>,
    mut state: ResMut<SetState<AppState>>,
    textures: Res<GameMaterials>,
) {
    for (mut handle, damage) in query.iter_mut() {
        if damage.0 >= PLAYER_HEALTH {
            state.schedule_operation(ScheduledOperation::Push(AppState::GameOver));
        } else {
            *handle = textures.disc[damage.0].clone();
        }
    }
}
