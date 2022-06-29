use crate::voxel::{
    materials::Rock,
    terraingen::{noise::NoiseMap, BiomeTerrainGenerator},
    ChunkKey, Voxel, CHUNK_LENGTH, CHUNK_LENGTH_U,
};

pub struct HeightmapBiomeTerrainGenerator {
    pub voxel: Voxel,
    pub biome_ord: f32,
}

impl HeightmapBiomeTerrainGenerator {
    const DEFAULT_TERRAIN_HEIGHT: i32 = 64;

    pub const fn new(voxel: Voxel, biome_ord: f32) -> Self {
        Self { voxel, biome_ord }
    }

    #[inline]
    fn heightmap_scale_func(x: f32, chunk_key: ChunkKey) -> u32 {
        ((Self::DEFAULT_TERRAIN_HEIGHT as i32 + ((x * 3.0).round() as i32))
            - chunk_key.location().y as i32)
            .max(0)
            .min((CHUNK_LENGTH) as i32) as u32
    }

    fn make_world_bottom_border(
        &self,
        buffer: &mut crate::voxel::storage::VoxelBuffer<Voxel, crate::voxel::ChunkShape>,
    ) {
        for x in 0..CHUNK_LENGTH {
            for z in 0..CHUNK_LENGTH {
                *buffer.voxel_at_mut([x, 0, z].into()) = Voxel(Rock::ID);
            }
        }
    }
}

impl BiomeTerrainGenerator for HeightmapBiomeTerrainGenerator {
    fn generate_terrain(
        &self,
        chunk_key: crate::voxel::ChunkKey,
        heightmap: NoiseMap<f32, CHUNK_LENGTH_U, CHUNK_LENGTH_U>,
        buffer: &mut crate::voxel::storage::VoxelBuffer<Voxel, crate::voxel::ChunkShape>,
    ) {
        for x in 0..CHUNK_LENGTH {
            for z in 0..CHUNK_LENGTH {
                let height = heightmap.map(x as usize, z as usize, |x| {
                    Self::heightmap_scale_func(x, chunk_key)
                });

                for h in 0..height {
                    *buffer.voxel_at_mut([x, h, z].into()) = self.voxel;
                }
            }
        }

        if chunk_key.location().y == 0 {
            self.make_world_bottom_border(buffer);
        }
    }

    fn biome_temp_humidity(&self) -> float_ord::FloatOrd<f32> {
        float_ord::FloatOrd(self.biome_ord)
    }
}