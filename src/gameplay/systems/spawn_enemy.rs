use bevy::prelude::*;
use crate::gameplay::*;

#[derive(Default)]
pub struct SpawnTimer(f32);
pub struct NextWait(f32);

impl Default for NextWait {
    fn default() -> Self {
        NextWait(rand::thread_rng().gen_range(-1.0..3.0))
    }
}

pub fn spawn_enemy_system(
    commands: &mut Commands,
    time: Res<Time>,
    textures: Res<GameMaterials>,
    mut enemy_count: ResMut<EnemyCount>,
    mut spawn_timer: Local<SpawnTimer>,
    mut next_wait: Local<NextWait>,
) {
    if enemy_count.0 < 8 {
        spawn_timer.0 += time.delta_seconds();
    }
    if spawn_timer.0 > next_wait.0 {
        spawn_timer.0 -= next_wait.0;
        let mut random = rand::thread_rng();

        next_wait.0 = random.gen_range(-1.0..3.0);

        let alpha = random.gen_range(0.0..std::f32::consts::TAU);
        let x = f32::cos(alpha);
        let y = f32::sin(alpha);
        let spawn_pos = Vec2::new(x, y) * SPAWN_DIST;

        enemy_count.0 += 1;

        commands
            .spawn(SpriteBundle {
                material: textures.drone[0].clone(),
                transform: Transform::from_translation(spawn_pos.extend(50.)),
                ..Default::default()
            })
            .with(Enemy)
            .with(Ineffective(Timer::new(Duration::from_millis(500), false)))
            .with(Velocity::default())
            .with(Damage(0));
    }
}