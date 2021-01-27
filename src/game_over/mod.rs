use bevy::prelude::*;

use crate::{AppState, STATE_STAGE};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.stage(STATE_STAGE, |state: &mut StateStage<AppState>| {
            state.on_state_enter(AppState::GameOver, (|| ()).system())
        });
    }
}
