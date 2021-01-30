use crate::game_over::*;
use bevy::prelude::*;

pub fn setup_system(commands: &mut Commands, textures: Res<GameMaterials>) {
    commands
        .spawn(NodeBundle {
            material: textures.drone_maker.clone(),
            style: Style {
                
                ..Default::default()
            },
            ..Default::default()
        })
        .with(GameOverEntity);
}
