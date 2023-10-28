use bevy::{prelude::*, math::ivec2};
use noise::{Fbm, Perlin, NoiseFn};
use rand::prelude::*;

use crate::states::GameState;

use super::{storage::{WorldStorage, WORLD_BLOCK_SIZE}, block::Block};

const SURFACE_LENGTH: f64 = 48.0;
const SURFACE_HEIGHT: f64 = 12.0;
const SURFACE_OFFSET: f64 = 30.0;

const STONE_LENGTH: f32 = 0.4;
const STONE_HEIGHT: f32 = 1.6;
const STONE_OFFSET: f32 = 50.0;
const STONE_THRESHOLD: usize = 5;

const CAVES_SCALE: f64 = 10.0;
const CAVES_TRESHOLD: f64 = -0.1;

pub fn generate(mut commands: Commands) {
    let mut world = WorldStorage::new();
    let mut rng = thread_rng();

    fill_dirt(&mut world);
    // carve_surface(&mut world, &mut rng);
    // fill_stone(&mut world, &mut rng);
    // carve_caves(&mut world, &mut rng);

    commands.insert_resource(world);
    commands.insert_resource(NextState(Some(GameState::InGame)));
}

fn fill_dirt(world: &mut WorldStorage) {
    for x in 0..WORLD_BLOCK_SIZE.x {
        for y in 0..WORLD_BLOCK_SIZE.y {
            world.set_block(ivec2(x, y), Block::Dirt);
            world.set_wall(ivec2(x, y), Block::Dirt);
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
        world.set_wall(ivec2(x, val), Block::Dirt);

        for y in (val + 1)..WORLD_BLOCK_SIZE.y {
            world.set_block(ivec2(x, y), Block::Air);
            world.set_wall(ivec2(x, y), Block::Air);
        }
    }
}

fn fill_stone(
    world: &mut WorldStorage,
    rng: &mut ThreadRng,
) {
    for x in 0..WORLD_BLOCK_SIZE.x {
        let val = ((x as f32 * STONE_LENGTH).sin() * STONE_HEIGHT + WORLD_BLOCK_SIZE.y as f32 - STONE_OFFSET) as usize;

        for y in (0..val).rev() {
            if y < val - STONE_THRESHOLD {
                world.set_block(ivec2(x, y as i32), Block::Stone);
                world.set_wall(ivec2(x, y as i32), Block::Stone);
                continue;
            }

            let block = if rng.gen_bool(0.5) { Block::Dirt } else { Block::Stone };
            world.set_block(ivec2(x, y as i32), block);
        }
    }
}

fn carve_caves(
    world: &mut WorldStorage,
    rng: &mut ThreadRng,
) {
    let fbm = Fbm::<Perlin>::new(rng.gen());

    for y in 0..WORLD_BLOCK_SIZE.y {
        for x in 0..WORLD_BLOCK_SIZE.x {
            if world.get_block(ivec2(x, y)).unwrap() != Block::Stone { continue; }

            let val = fbm.get([x as f64 / CAVES_SCALE, y as f64 / CAVES_SCALE, 0.0]);
            if val < CAVES_TRESHOLD {
                world.set_block(ivec2(x, y), Block::Air);
            }
        }
    }
}