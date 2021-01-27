use bevy::prelude::*;
use crate::gameplay::*;

pub fn rotate_staff_system(
    mut staff_rotator: Query<(&mut Transform, &GlobalTransform), With<StaffRotator>>,
    windows: Res<Windows>,
    camera: Query<&Transform, With<Camera>>,
) {
    let window = windows.get_primary().unwrap();
    let cursor_position = if let Some(position) = window.cursor_position() {
        position + camera.iter().next().unwrap().translation.truncate()
            - Vec2::new(window.width(), window.height()) / 2.
    } else {
        return;
    };

    let mut staff_rotator = staff_rotator.iter_mut().next().unwrap();

    let relative_pos = cursor_position - staff_rotator.1.translation.truncate();
    staff_rotator.0.rotation = Quat::from_rotation_z(
        f32::atan2(relative_pos.y, relative_pos.x) - std::f32::consts::FRAC_PI_2,
    );
}
