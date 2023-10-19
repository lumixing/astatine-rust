use bevy::{prelude::*, math::ivec2};
use noise::{Fbm, Perlin, NoiseFn};
use rand::prelude::*;

use crate::states::GameState;

use super::{storage::{WorldStorage, WORLD_BLOCK_SIZE}, block::Block};

const SURFACE_LENGTH: f64 = 24.0;
const SURFACE_HEIGHT: f64 = 12.0;
const SURFACE_OFFSET: f64 = 30.0;

pub fn generate(mut commands: Commands) {
    let mut world = WorldStorage::new();
    let mut rng = thread_rng();

    fill_dirt(&mut world);
    carve_surface(&mut world, &mut rng);

    commands.insert_resource(world);
    commands.insert_resource(NextState(Some(GameState::InGame)));
}

fn fill_dirt(world: &mut WorldStorage) {
    for x in 0..WORLD_BLOCK_SIZE.x {
        for y in 0..WORLD_BLOCK_SIZE.y {
            world.set_block(ivec2(x, y), Block::Dirt);
        }
    }
}

fn carve_surface(
    world: &mut WorldStorage,
    rng: &mut ThreadRng
) {
    let fbm = Fbm::<Perlin>::new(rng.gen());

    for x in 0..WORLD_BLOCK_SIZE.x {
        let val = (fbm.get([x as f64 / SURFACE_LENGTH, 0.0, 0.0]) * SURFACE_HEIGHT + WORLD_BLOCK_SIZE.y as f64 - SURFACE_OFFSET) as i32;
        world.set_block(ivec2(x, val), Block::Grass);

        for y in (val + 1)..WORLD_BLOCK_SIZE.y {
            world.set_block(ivec2(x, y), Block::Air);
        }
    }
}