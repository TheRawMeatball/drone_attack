use bevy::prelude::*;
use crate::gameplay::*;

pub fn target_highlight_system(
    closest_enemy: Res<ClosestEnemy>,
    mut query: Query<(&mut Transform, &mut Visible), With<TargetCircle>>,
    enemy_query: Query<&GlobalTransform, With<Enemy>>,
) {
    let mut circle = query.iter_mut().next().unwrap();

    if let Some(e) = closest_enemy.0 {
        let enemy_pos = enemy_query.get(e).unwrap().translation.truncate();

        circle.0.translation.x = enemy_pos.x;
        circle.0.translation.y = enemy_pos.y;
        circle.1.is_visible = true;
    } else {
        circle.1.is_visible = false;
    }
}