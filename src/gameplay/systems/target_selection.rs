use crate::gameplay::*;
use bevy::prelude::*;

pub fn target_selection_system(
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    camera: Query<&Transform, With<Camera>>,
    windows: Res<Windows>,
    mut closest_enemy: ResMut<ClosestEnemy>,
) {
    let window = windows.get_primary().unwrap();
    let cursor_position = if let Some(position) = window.cursor_position() {
        position + camera.iter().next().unwrap().translation.truncate()
            - Vec2::new(window.width(), window.height()) / 2.
    } else {
        return;
    };

    closest_enemy.0 = enemies
        .iter()
        .fold(
            (None, 128. * 128.),
            |(closest_enemy, min_dist), (enemy, transform)| {
                let enemy_pos = transform.translation.truncate();
                let dist = cursor_position.distance_squared(enemy_pos);

                if dist < min_dist {
                    (Some(enemy), dist)
                } else {
                    (closest_enemy, min_dist)
                }
            },
        )
        .0;
}
