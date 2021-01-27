use crate::gameplay::*;
pub use bevy::prelude::*;

pub fn enemy_collision_system(
    enemies: Query<(Entity, &Transform, &mut Velocity, &mut Damage), With<Enemy>>,
) {
    // This is safe - same entity is never mutabily accessed twice
    for (a, ..) in unsafe { enemies.iter_unsafe() } {
        for (b, ..) in unsafe { enemies.iter_unsafe() } {
            if a != b {
                let (_, transform_a, mut velocity_a, mut damage_a) =
                    unsafe { enemies.get_unsafe(a).unwrap() };
                let (_, transform_b, mut velocity_b, mut damage_b) =
                    unsafe { enemies.get_unsafe(b).unwrap() };
                let dist = transform_b.translation.truncate() - transform_a.translation.truncate();

                if dist.length_squared() < 64. * 64. {
                    let new_velocity = dist.normalize() * ENEMY_COLLISION_STRENGTH;
                    velocity_a.0 = -new_velocity;
                    velocity_b.0 = new_velocity;

                    damage_a.0 += 1;
                    damage_b.0 += 1;
                }
            }
        }
    }
}
