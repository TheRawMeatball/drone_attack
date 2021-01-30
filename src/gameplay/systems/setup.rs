use crate::gameplay::*;
use bevy::prelude::*;

pub fn setup_system(
    commands: &mut Commands,
    textures: Res<GameMaterials>,
) {
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::unit_z()),
            material: textures.disc[0].clone(),
            ..Default::default()
        })
        .with(Player)
        .with(GameplayEntity)
        .with(Damage(0))
        .with_children(|parent| {
            parent
                .spawn((
                    Transform::default(),
                    GlobalTransform::default(),
                    StaffRotator,
                ))
                .with(GameplayEntity)
                .with_children(|parent| {
                    parent
                        .spawn(SpriteBundle {
                            transform: Transform::from_translation(Vec3::new(0., 10., 1.)),
                            material: textures.staff.clone(),
                            ..Default::default()
                        })
                        .with(GameplayEntity);
                });
        });

    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::unit_z() * 2.),
            sprite: Sprite {
                size: Vec2::new(64., 64.),
                resize_mode: SpriteResizeMode::Manual,
            },
            material: textures.circle.clone(),
            ..Default::default()
        })
        .with(TargetCircle)
        .with(GameplayEntity);
}
