use bevy::{
    prelude::{info, Color, Plugin},
    utils::HashMap,
};
use bitflags::bitflags;
use std::{any::type_name, any::TypeId};

// Registry info about a voxel material
pub struct MaterialRegistryInfo {
    pub name: &'static str,
    pub base_color: Color,
    pub flags: VoxelMaterialFlags,
}

#[macro_export]
macro_rules! define_voxel_material {
    ($types: ident, $name: expr, $id: expr) => {
        pub struct $types;
        impl $types {
            pub const ID: u8 = $id;
            pub const NAME: &'static str = $name;
        }
    };
}

bitflags! {
    pub struct VoxelMaterialFlags : u32 {
        const SOLID = 0 << 0;
        const LIQUID = 1 << 1;
    }
}

/// A registry for voxel material types.
/// This stores the voxel materials along their material id used to refer them in voxel data
pub struct VoxelMaterialRegistry {
    materials: Vec<MaterialRegistryInfo>,
    mat_ids: HashMap<TypeId, usize>,
}

#[allow(dead_code)]
impl VoxelMaterialRegistry {
    #[inline]
    pub fn get_by_id(&self, id: u8) -> Option<&MaterialRegistryInfo> {
        self.materials.get(id as usize)
    }

    pub fn get_mut_by_id(&mut self, id: u8) -> Option<&mut MaterialRegistryInfo> {
        self.materials.get_mut(id as usize)
    }

    pub fn get_by_type<M: 'static>(&self) -> Option<&MaterialRegistryInfo> {
        self.mat_ids
            .get(&TypeId::of::<M>())
            .map(|x| self.materials.get(*x).unwrap())
    }

    pub fn get_id_for_type<M: 'static>(&self) -> Option<u8> {
        self.mat_ids.get(&TypeId::of::<M>()).map(|x| *x as u8)
    }

    pub fn register_material<M: 'static>(&mut self, mat: MaterialRegistryInfo) {
        self.materials.push(mat);
        info!(
            "Registered material {:?} (ID: {})",
            type_name::<M>(),
            self.materials.len() - 1
        );
        self.mat_ids.insert(TypeId::of::<M>(), self.materials.len());
    }

    pub fn iter_mats(&self) -> impl Iterator<Item = &MaterialRegistryInfo> {
        self.materials.iter()
    }
}

impl Default for VoxelMaterialRegistry {
    fn default() -> Self {
        let mut registry = Self {
            materials: Default::default(),
            mat_ids: Default::default(),
        };

        registry.register_material::<Void>(MaterialRegistryInfo {
            base_color: Color::BLACK,
            name: "Void",
            flags: VoxelMaterialFlags::SOLID,
        });

        registry
    }
}

// The material with ID #0;
pub struct Void;

pub struct VoxelMaterialPlugin;
impl Plugin for VoxelMaterialPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<VoxelMaterialRegistry>();
    }
}