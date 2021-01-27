use bevy::prelude::*;
use crate::gameplay::*;

pub fn player_health_system(
    mut query: Query<(&mut Handle<ColorMaterial>, &Damage), (Changed<Damage>, With<Player>)>,
    mut state: ResMut<State<AppState>>,
    textures: Res<GameMaterials>,
) {
    for (mut handle, damage) in query.iter_mut() {
        if damage.0 >= PLAYER_HEALTH {
            state.set_next(AppState::GameOver).unwrap();
        } else {
            *handle = textures.disc[damage.0].clone();
        }
    }
}