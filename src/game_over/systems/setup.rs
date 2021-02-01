use crate::game_over::*;
use bevy::prelude::*;
use bevy_ninepatch_shader::NinepatchBundle;

pub fn setup_system(commands: &mut Commands, textures: Res<GameMaterials>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: textures.none.clone(),
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn(NinepatchBundle {
                style: Style {
                    size: Size::new(Val::Px(500.0), Val::Px(500.0)),
                    ..Default::default()
                },
                material: textures.bordered_ui.clone(),
                ..Default::default()
            })
            .with(GameOverEntity);
        })
        .with(GameOverEntity);
}
