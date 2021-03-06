#[derive(Clone, Copy, Debug)]
pub enum GroundMaterial {
    Dirt,
    Grass,
    Stone,
    Sand,
    Snow,
    JungleGrass,
    Mulch,
    Debug,
}

impl GroundMaterial {
    // Returns color of Texture in RGB
    pub fn get_color(&self) -> [f32; 3] {
        match *self {
            GroundMaterial::Dirt => [0.395, 0.26, 0.13],
            GroundMaterial::Grass => [0.0, 0.5, 0.0],
            GroundMaterial::Stone => [0.5, 0.5, 0.5],
            GroundMaterial::Snow => [0.95, 0.95, 1.0],
            GroundMaterial::Sand => [0.945, 0.86, 0.49],
            GroundMaterial::JungleGrass => [0.1, 0.26, 0.04],
            GroundMaterial::Mulch => [0.332, 0.219, 0.109],
            GroundMaterial::Debug => [1.0, 0.0, 0.0],
        }
    }

    pub fn get_id(&self) -> i32 {
        match *self {
            GroundMaterial::Grass => 1,
            GroundMaterial::Sand => 2,
            GroundMaterial::Snow => 3,
            GroundMaterial::Dirt => 4,
            GroundMaterial::Stone => 5,
            GroundMaterial::JungleGrass => 1,
            GroundMaterial::Mulch => 7,
            GroundMaterial::Debug => 8,
        }
    }
}
