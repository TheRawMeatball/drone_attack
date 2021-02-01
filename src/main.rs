mod asset_manager;
mod game_over;
mod gameplay;

pub use asset_manager::*;
use bevy::{
    ecs::{SetState, StateSetBuilder},
    prelude::*,
};
use bevy_ninepatch_shader::NinepatchUIShaderPlugin;
use game_over::GameOverPlugin;
use gameplay::GameplayPlugin;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AppState {
    Loading,
    Running,
    GameOver,
    Paused,
    MainMenu,
}

pub use gameplay::{DRONE_HEALTH, PLAYER_HEALTH};

fn main() {
    let mut state = StateSetBuilder::<AppState>::default();
    state.add_on_update(AppState::Loading, load_poll_system.system());
    state.add_on_exit(AppState::Loading, on_load_end.system());

    GameplayPlugin::add_systems(&mut state);
    GameOverPlugin::add_systems(&mut state);

    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(NinepatchUIShaderPlugin)
        .add_resource(SetState::new(AppState::Loading))
        .init_resource::<GameTextures>()
        .init_resource::<GameSfx>()
        .init_resource::<GameMusic>()
        .init_resource::<GameMeshes>()
        .add_plugin(GameplayPlugin)
        .add_plugin(GameOverPlugin)
        .stage(stage::UPDATE, |stage: &mut SystemStage| {
            state.finalize(stage);
            stage
        })
        .add_system(music_system.system())
        .run();
}

pub struct Background;

fn on_load_end(commands: &mut Commands, meshes: Res<GameMeshes>, textures: Res<GameMaterials>) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                size: Vec2::new(64., 64.),
                resize_mode: SpriteResizeMode::Manual,
            },
            mesh: meshes.plate.clone(),
            material: textures.plate.clone(),
            ..Default::default()
        })
        .with(Background)
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
}

fn music_system(
    music_player: ResMut<Audio>,
    audio_assets: Res<Assets<AudioSource>>,
    tracks: Res<GameMusic>,
    time: Res<Time>,
    mut active_track: Local<usize>,
    mut until_done: Local<Timer>,
    state: Res<SetState<AppState>>,
) {
    if *state.current() == AppState::Loading {
        *active_track = tracks.tracks.len() - 1;
        until_done.set_duration(0.);
        return;
    }
    until_done.tick(time.delta_seconds());
    if until_done.finished() {
        use rodio::Source;
        *active_track = (*active_track + 1) % tracks.tracks.len();
        let track = tracks.tracks[*active_track].clone();
        let duration = audio_assets
            .get(&track)
            .unwrap()
            .decoder()
            .total_duration()
            .unwrap();
        until_done.set_duration(duration.as_secs_f32());
        until_done.reset();
        music_player.play(track.clone());
    }
}
