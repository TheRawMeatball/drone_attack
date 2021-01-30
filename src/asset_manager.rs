use std::convert::TryInto;

use bevy::{
    asset::LoadState,
    audio::AudioSource,
    ecs::{Commands, FromResources, Res, ResMut, Resources, SetState},
    math::Vec2,
    prelude::{AssetServer, Assets, Color, ColorMaterial, Handle, Mesh, Texture},
};

use crate::{AppState, DRONE_HEALTH, PLAYER_HEALTH};

pub struct GameTextures {
    pub circle: Handle<Texture>,
    pub drone: Handle<Texture>,
    pub drone_maker: Handle<Texture>,
    pub disc: Handle<Texture>,
    pub plate: Handle<Texture>,
    pub staff: Handle<Texture>,
}
pub struct GameMaterials {
    pub circle: Handle<ColorMaterial>,
    pub drone: [Handle<ColorMaterial>; DRONE_HEALTH],
    pub drone_maker: Handle<ColorMaterial>,
    pub disc: [Handle<ColorMaterial>; PLAYER_HEALTH],
    pub plate: Handle<ColorMaterial>,
    pub staff: Handle<ColorMaterial>,
}

impl GameTextures {
    const PATHS: [&'static str; 6] = [
        "textures/circle.png",
        "textures/drone.png",
        "textures/drone_maker.png",
        "textures/disc.png",
        "textures/plate.png",
        "textures/staff.png",
    ];
}

impl FromResources for GameTextures {
    fn from_resources(resources: &Resources) -> Self {
        let asset_server = resources.get::<AssetServer>().unwrap();
        let mut iterator = Self::PATHS.iter().map(|path| asset_server.load(*path));

        Self {
            circle: iterator.next().unwrap(),
            drone: iterator.next().unwrap(),
            drone_maker: iterator.next().unwrap(),
            disc: iterator.next().unwrap(),
            plate: iterator.next().unwrap(),
            staff: iterator.next().unwrap(),
        }
    }
}

pub struct GameSfx {
    pub explosion: Handle<AudioSource>,
    pub teleport: Handle<AudioSource>,
}

impl GameSfx {
    const PATHS: [&'static str; 2] = ["sfx/explosion.mp3", "sfx/teleport.mp3"];
}

impl FromResources for GameSfx {
    fn from_resources(resources: &Resources) -> Self {
        let asset_server = resources.get::<AssetServer>().unwrap();

        let mut iterator = Self::PATHS.iter().map(|path| asset_server.load(*path));

        Self {
            explosion: iterator.next().unwrap(),
            teleport: iterator.next().unwrap(),
        }
    }
}

pub struct GameMusic {
    pub tracks: Vec<Handle<AudioSource>>,
}

impl FromResources for GameMusic {
    fn from_resources(resources: &Resources) -> Self {
        let asset_server = resources.get::<AssetServer>().unwrap();

        let tracks = asset_server
            .load_folder("music")
            .unwrap()
            .into_iter()
            .map(|h| h.typed())
            .collect();

        Self { tracks }
    }
}

pub struct GameMeshes {
    pub plate: Handle<Mesh>,
}

impl FromResources for GameMeshes {
    fn from_resources(resources: &Resources) -> Self {
        let mut meshes = resources.get_mut::<Assets<Mesh>>().unwrap();

        Self {
            plate: meshes.add(Mesh::from(TileQuad::new(
                Vec2::new(100.0, 100.0),
                Vec2::new(100., 100.),
            ))),
        }
    }
}

pub struct TileQuad {
    pub size: Vec2,
    pub flip: bool,
    pub tiling: Vec2,
}

impl TileQuad {
    pub fn new(size: Vec2, tiling: Vec2) -> Self {
        Self {
            size,
            flip: false,
            tiling,
        }
    }

    pub fn flipped(size: Vec2, tiling: Vec2) -> Self {
        Self {
            size,
            flip: true,
            tiling,
        }
    }
}

impl From<TileQuad> for Mesh {
    fn from(quad: TileQuad) -> Self {
        let extent_x = quad.size.x / 2.0;
        let extent_y = quad.size.y / 2.0;

        let north_west = Vec2::new(-extent_x, extent_y);
        let north_east = Vec2::new(extent_x, extent_y);
        let south_west = Vec2::new(-extent_x, -extent_y);
        let south_east = Vec2::new(extent_x, -extent_y);
        let vertices = if quad.flip {
            [
                (
                    [south_east.x, south_east.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [quad.tiling.x, quad.tiling.y],
                ),
                (
                    [north_east.x, north_east.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [quad.tiling.x, 0.0],
                ),
                (
                    [north_west.x, north_west.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [0.0, 0.0],
                ),
                (
                    [south_west.x, south_west.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [0.0, quad.tiling.y],
                ),
            ]
        } else {
            [
                (
                    [south_west.x, south_west.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [0.0, quad.tiling.y],
                ),
                (
                    [north_west.x, north_west.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [0.0, 0.0],
                ),
                (
                    [north_east.x, north_east.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [quad.tiling.x, 0.0],
                ),
                (
                    [south_east.x, south_east.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [quad.tiling.x, quad.tiling.y],
                ),
            ]
        };

        let indices = bevy::render::mesh::Indices::U32(vec![0, 2, 1, 0, 3, 2]);

        let mut positions = Vec::<[f32; 3]>::new();
        let mut normals = Vec::<[f32; 3]>::new();
        let mut uvs = Vec::<[f32; 2]>::new();
        for (position, normal, uv) in vertices.iter() {
            positions.push(*position);
            normals.push(*normal);
            uvs.push(*uv);
        }

        let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}

pub fn load_poll_system(
    server: Res<AssetServer>,
    textures: Res<GameTextures>,
    music: Res<GameMusic>,
    sfx: Res<GameSfx>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_sources: ResMut<Assets<Texture>>,
    mut state: ResMut<SetState<AppState>>,
    commands: &mut Commands,
) {
    let done_loading = music
        .tracks
        .iter()
        .all(|track| server.get_load_state(track.id) == LoadState::Loaded)
        && server.get_load_state(textures.circle.id) == LoadState::Loaded
        && server.get_load_state(textures.drone.id) == LoadState::Loaded
        && server.get_load_state(textures.drone_maker.id) == LoadState::Loaded
        && server.get_load_state(textures.disc.id) == LoadState::Loaded
        && server.get_load_state(textures.plate.id) == LoadState::Loaded
        && server.get_load_state(textures.staff.id) == LoadState::Loaded
        && server.get_load_state(sfx.explosion.id) == LoadState::Loaded
        && server.get_load_state(sfx.teleport.id) == LoadState::Loaded;

    if done_loading {
        commands.spawn(bevy::prelude::Camera2dBundle::default());

        commands.insert_resource(GameMaterials {
            circle: materials.add(ColorMaterial::texture(textures.circle.clone())),
            drone: (0..DRONE_HEALTH)
                .map(|i| {
                    materials.add(ColorMaterial::modulated_texture(textures.drone.clone(), {
                        let diff = 1. - i as f32 / DRONE_HEALTH as f32;
                        Color::rgb(1., diff, diff)
                    }))
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            drone_maker: materials.add(ColorMaterial::texture(textures.drone_maker.clone())),
            disc: (0..PLAYER_HEALTH)
                .map(|i| {
                    materials.add(ColorMaterial::modulated_texture(textures.disc.clone(), {
                        let diff = 1. - i as f32 / DRONE_HEALTH as f32;
                        Color::rgb(1., diff, diff)
                    }))
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            plate: materials.add(ColorMaterial::texture(textures.plate.clone())),
            staff: materials.add(ColorMaterial::texture(textures.staff.clone())),
        });

        texture_sources
            .get_mut(textures.plate.id)
            .unwrap()
            .sampler
            .set_address_mode(bevy::render::texture::AddressMode::Repeat);

        state.schedule_operation(bevy::ecs::ScheduledOperation::Next(AppState::Running));
    }
}
