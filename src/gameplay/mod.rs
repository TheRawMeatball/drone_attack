mod components;
mod resources;
mod systems;

use std::time::Duration;

use bevy::{ecs::StateSetBuilder, prelude::*, render::camera::Camera};
use rand::Rng;

use crate::{AppState, GameMaterials, GameSfx};

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

impl GameplayPlugin {
    pub fn add_systems(builder: &mut StateSetBuilder<AppState>) {
        builder
            .add_on_enter(AppState::Running, setup_system.system())
            .add_on_exit(AppState::Running, cleanup_system.system())
            .add_on_update(AppState::Running, rotate_staff_system.system())
            .add_on_update(
                AppState::Running,
                spawn_enemy_system.system().after("PlayerCollision"),
            )
            .add_on_update(
                AppState::Running,
                target_selection_system.system().label("TargetSelect"),
            )
            .add_on_update(
                AppState::Running,
                enemy_collision_system
                    .system()
                    .label("EnemyCollision")
                    .before("Velocity"),
            )
            .add_on_update(
                AppState::Running,
                target_highlight_system.system().after("TargetSelect"),
            )
            .add_on_update(
                AppState::Running,
                player_collision_system
                    .system()
                    .label("PlayerCollision")
                    .before("Velocity")
                    .after("Teleport"),
            )
            .add_on_update(
                AppState::Running,
                teleport_system
                    .system()
                    .label("Teleport")
                    .after("TargetSelect")
                    .before("Velocity"),
            )
            .add_on_update(
                AppState::Running,
                drone_health_system
                    .system()
                    .after("Teleport")
                    .after("EnemyCollision"),
            )
            .add_on_update(
                AppState::Running,
                player_health_system.system().after("PlayerCollision"),
            )
            .add_on_update(
                AppState::Running,
                enemy_ai_system
                    .system()
                    .label("EnemyAi")
                    .before("Velocity")
                    .after("PlayerCollision")
                    .after("EnemyCollision")
                    .after("Teleport"),
            )
            .add_on_update(
                AppState::Running,
                velocity_system.system().label("Velocity"),
            );
    }
}

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<EnemyCount>()
            .init_resource::<ClosestEnemy>();
    }
}
