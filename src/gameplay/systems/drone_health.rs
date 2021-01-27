use crate::gameplay::*;
use bevy::prelude::*;

pub fn drone_health_system(
    mut query: Query<(Entity, &mut Handle<ColorMaterial>, &Damage), (Changed<Damage>, With<Enemy>)>,
    textures: Res<GameMaterials>,
    mut enemy_count: ResMut<EnemyCount>,
    commands: &mut Commands,
    sfx: Res<GameSfx>,
    sfx_player: ResMut<Audio>,
) {
    for (entity, mut handle, damage) in query.iter_mut() {
        if damage.0 >= DRONE_HEALTH {
            commands.despawn(entity);
            enemy_count.0 -= 1;
            sfx_player.play(sfx.explosion.clone());
        } else {
            *handle = textures.drone[damage.0].clone();
        }
    }
}
