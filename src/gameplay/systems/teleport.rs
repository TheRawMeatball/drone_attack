use bevy::prelude::*;
use crate::gameplay::*;

pub fn teleport_system(
    closest_enemy: Res<ClosestEnemy>,
    mut enemies: Query<
        (
            Entity,
            &Transform,
            &mut Velocity,
            &mut Damage,
            &mut Ineffective,
        ),
        With<Enemy>,
    >,
    inputs: Res<Input<MouseButton>>,
    mut player: Query<&mut Transform, With<Player>>,
    sounds: Res<GameSfx>,
    sfx_player: ResMut<Audio>,
) {
    if inputs.just_pressed(MouseButton::Left) {
        if let Some(e) = closest_enemy.0 {
            let player_pos = &mut player.iter_mut().next().unwrap().translation;
            let (_, enemy_transform, mut velocity, mut damage, mut ineffective) =
                enemies.get_mut(e).unwrap();
            let enemy_pos = enemy_transform.translation;
            ineffective.0.reset();
            velocity.0 =
                (enemy_pos.truncate() - player_pos.truncate()).normalize() * TELEPORT_PUSH_STRENGTH;
            damage.0 += 1;
            player_pos.x = enemy_pos.x;
            player_pos.y = enemy_pos.y;

            sfx_player.play(sounds.teleport.clone());
        }
    }
}