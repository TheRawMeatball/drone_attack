mod asset_manager;
mod game_over;
mod gameplay;

pub use asset_manager::*;
use bevy::prelude::*;
use game_over::GameOverPlugin;
use gameplay::GameplayPlugin;

#[derive(Clone, PartialEq, Eq)]
pub enum AppState {
    Loading,
    Running,
    GameOver,
}

const STATE_STAGE: &str = "DRONE_ATTACK";

pub use gameplay::{DRONE_HEALTH, PLAYER_HEALTH};

fn main() {
    let mut state = StateStage::<AppState>::default();
    state.on_state_update(AppState::Loading, load_poll_system.system());

    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(State::new(AppState::Loading))
        .init_resource::<GameTextures>()
        .init_resource::<GameSfx>()
        .init_resource::<GameMusic>()
        .init_resource::<GameMeshes>()
        .add_stage_after(stage::UPDATE, STATE_STAGE, state)
        .add_plugin(GameplayPlugin)
        .add_plugin(GameOverPlugin)
        .add_system(music_system.system())
        .run();
}

fn music_system(
    music_player: ResMut<Audio>,
    audio_assets: Res<Assets<AudioSource>>,
    tracks: Res<GameMusic>,
    time: Res<Time>,
    mut active_track: Local<usize>,
    mut until_done: Local<Timer>,
    state: Res<State<AppState>>,
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
