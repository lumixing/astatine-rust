#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
#[repr(u32)]
#[allow(dead_code)]
pub enum Block {
    Air,
    Grass,
    Dirt,
    Stone,
    Border,
}

impl Block {
    #[allow(dead_code)]
    pub fn from(block: u32) -> Self {
        match block {
            0 => Self::Air,
            1 => Self::Grass,
            2 => Self::Dirt,
            3 => Self::Stone,
            4 => Self::Border,
            _ => Self::Air,
        }
    }

    pub fn should_flip(&self) -> bool {
        match self {
            Block::Air => false,
            Block::Grass => false,
            Block::Dirt => true,
            Block::Stone => true,
            Block::Border => true,
        }
    }

    pub fn texture_path(&self) -> &str {
        match self {
            Block::Air => "tiles/air.png",
            Block::Grass => "tiles/grass.png",
            Block::Dirt => "tiles/dirt.png",
            Block::Stone => "tiles/stone.png",
            Block::Border => "tiles/border.png",
        }
    }
}
