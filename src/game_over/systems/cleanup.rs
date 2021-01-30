use crate::game_over::*;
use bevy::prelude::*;

pub fn cleanup_system(commands: &mut Commands, despawned: Query<Entity, With<GameOverEntity>>) {
    for entity in despawned.iter() {
        commands.despawn_recursive(entity);
    }
}
