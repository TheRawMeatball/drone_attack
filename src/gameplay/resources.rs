use bevy::prelude::*;

#[derive(Default)]
pub struct EnemyCount(pub usize);
#[derive(Default)]
pub struct ClosestEnemy(pub Option<Entity>);