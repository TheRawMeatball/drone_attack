use bevy::prelude::*;
use crate::gameplay::*;

pub fn player_collision_system(
    mut player: Query<(&Transform, &mut Damage), With<Player>>,
    mut enemies: Query<(&Transform, &mut Velocity, &mut Ineffective), With<Enemy>>,
    time: Res<Time>,
) {
    let (player_pos, mut player_damage) = player
        .iter_mut()
        .next()
        .map(|(transform, damage)| (transform.translation.truncate(), damage))
        .unwrap();

    for (transform, mut velocity, mut ineffective) in enemies.iter_mut() {
        let enemy_pos = transform.translation.truncate();
        let player_relative_pos = enemy_pos - player_pos;
        ineffective.0.tick(time.delta_seconds());
        if player_relative_pos.length_squared() < 32. * 32. {
            let direction = player_relative_pos.normalize();
            if direction.is_nan() {
                continue;
            }
            velocity.0 = direction * PLAYER_COLLISION_STRENGTH;
            if ineffective.0.finished() {
                player_damage.0 += 1;
            }
        }
    }
}