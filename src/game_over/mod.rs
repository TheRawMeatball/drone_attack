mod components;
mod resources;
mod systems;

use bevy::{ecs::StateSetBuilder, prelude::*};

use crate::{AppState, GameMaterials};

use components::*;
use resources::*;
use systems::*;

pub struct GameOverPlugin;

impl GameOverPlugin {
    pub fn add_systems(builder: &mut StateSetBuilder<AppState>) {
        builder
            .add_on_enter(AppState::GameOver, setup_system.system())
            .add_on_exit(AppState::GameOver, cleanup_system.system());
    }
}

impl Plugin for GameOverPlugin {
    fn build(&self, _app: &mut AppBuilder) {}
}
