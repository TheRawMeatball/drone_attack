use crate::gameplay::*;
use bevy::prelude::*;

pub fn enemy_ai_system(
    player: Query<&Transform, With<Player>>,
    mut enemies: Query<(&Transform, &mut Velocity), With<Enemy>>,
    time: Res<Time>,
) {
    let player_pos = player.iter().next().unwrap().translation.truncate();

    for (transform, mut velocity) in enemies.iter_mut() {
        let dist: Vec2 = player_pos - transform.translation.truncate();
        let target_velocity = dist.normalize() * DRONE_SPEED;
        if target_velocity.is_nan() {
            continue;
        }
        let delta_velocity = (target_velocity - velocity.0) * time.delta_seconds() * 5.;
        velocity.0 += delta_velocity;
    }
}
