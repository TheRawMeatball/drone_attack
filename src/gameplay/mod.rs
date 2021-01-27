mod components;
mod resources;
mod systems;

use std::time::Duration;

use bevy::{prelude::*, render::camera::Camera};
use rand::Rng;

use crate::{AppState, GameMaterials, GameMeshes, GameSfx, STATE_STAGE};

use components::*;
use resources::*;
use systems::*;

pub const DRONE_HEALTH: usize = 3;
pub const PLAYER_HEALTH: usize = 10;

const PLAYER_COLLISION_STRENGTH: f32 = 750.;
const ENEMY_COLLISION_STRENGTH: f32 = 250.;
const TELEPORT_PUSH_STRENGTH: f32 = 750.;
const DRONE_SPEED: f32 = 75.;
const SPAWN_DIST: f32 = 128. * 3.;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.stage(STATE_STAGE, |state: &mut StateStage<AppState>| {
            state
                .on_state_enter(AppState::Running, setup_system.system())
                .on_state_exit(AppState::Running, cleanup_system.system())
                .on_state_update(AppState::Running, rotate_staff_system.system())
                .on_state_update(
                    AppState::Running,
                    spawn_enemy_system.system().after("PlayerCollision"),
                )
                .on_state_update(
                    AppState::Running,
                    target_selection_system.system().label("TargetSelect"),
                )
                .on_state_update(
                    AppState::Running,
                    enemy_collision_system
                        .system()
                        .label("EnemyCollision")
                        .before("Velocity"),
                )
                .on_state_update(
                    AppState::Running,
                    target_highlight_system.system().after("TargetSelect"),
                )
                .on_state_update(
                    AppState::Running,
                    player_collision_system
                        .system()
                        .label("PlayerCollision")
                        .before("Velocity")
                        .after("Teleport"),
                )
                .on_state_update(
                    AppState::Running,
                    teleport_system
                        .system()
                        .label("Teleport")
                        .after("TargetSelect")
                        .before("Velocity"),
                )
                .on_state_update(
                    AppState::Running,
                    drone_health_system
                        .system()
                        .after("Teleport")
                        .after("EnemyCollision"),
                )
                .on_state_update(
                    AppState::Running,
                    player_health_system.system().after("PlayerCollision"),
                )
                .on_state_update(
                    AppState::Running,
                    enemy_ai_system
                        .system()
                        .label("EnemyAi")
                        .before("Velocity")
                        .after("PlayerCollision")
                        .after("EnemyCollision")
                        .after("Teleport"),
                )
                .on_state_update(
                    AppState::Running,
                    velocity_system.system().label("Velocity"),
                )
        })
        .init_resource::<EnemyCount>()
        .init_resource::<ClosestEnemy>();
    }
}
