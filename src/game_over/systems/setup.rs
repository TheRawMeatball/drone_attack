use crate::game_over::*;
use bevy::prelude::*;

pub fn setup_system(commands: &mut Commands, textures: Res<GameMaterials>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(200.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(210.0),
                    bottom: Val::Px(10.0),
                    ..Default::default()
                },
                border: Rect::all(Val::Px(20.0)),
                ..Default::default()
            },
            material: textures.drone_maker.clone(),
            ..Default::default()
        })
        .with(GameOverEntity);
}
