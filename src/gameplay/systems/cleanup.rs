use crate::gameplay::*;
use bevy::prelude::*;

pub fn cleanup_system(commands: &mut Commands, despawned: Query<Entity, With<GameplayEntity>>) {
    for entity in despawned.iter() {
        commands.despawn(entity);
    }
}
