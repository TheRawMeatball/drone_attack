use bevy::prelude::*;
use crate::gameplay::*;
pub fn velocity_system(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0.extend(0.) * time.delta_seconds();
    }
}