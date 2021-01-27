use crate::gameplay::*;
use bevy::prelude::*;

pub fn cleanup_system(
    commands: &mut Commands,
    despawned: Query<
        Entity,
        Or<(
            With<Player>,
            With<Enemy>,
            With<TargetCircle>,
            With<Background>,
        )>,
    >,
) {
    for entity in despawned.iter() {
        commands.despawn_recursive(entity);
    }
}
