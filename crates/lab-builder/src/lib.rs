use bevy::{prelude::*};

pub mod systems;
pub mod maps;
pub mod text;

use systems::*;
use lab_core;
use lab_entities::prelude::*;
use lab_sprites::SpriteInfo;

pub mod prelude {
    pub use systems::*;
    pub use maps::*;
    pub use text::*;
    pub use crate::*;
}

pub enum RelativePosition {
    LeftOf,
    RightOf,
    Above,
    Below,
    Current
}

pub struct BuilderPlugin; 

impl Plugin for BuilderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .init_resource::<BuilderSettings>()
        // system to init the tile palette
        .add_startup_system_to_stage(lab_core::stage::POST_INIT, make_tile_palette_system.system())
        // system that will add tiles on click
        .add_system_to_stage(lab_core::stage::PRE_UPDATE, add_tiles_to_world_system.system())
        .add_system(builder_keyboard_system.system())
        .add_system(update_tile_system.system())
        // System for changing builder settings
        .add_system(builder_settings_system.system());
    }
}

#[derive(Default)]
pub struct BuilderSettings {
    pub move_mode: bool
}

/// Mark a tile as moving (i.e. being dragged)
pub struct MovingTile;

#[derive(Bundle, Clone, Default, Debug)]
pub struct MobComponents {
    pub npc: NonPlayer,
    pub movement: Movement,
    pub sprite: SpriteInfo,
    pub inventory: Inventory,
    pub attributes: lab_world::TileAttributes,
    pub interaction: lab_world::Interaction,
    pub timer : Timer,
    pub location : Location,
    pub zoomable : lab_core::Zoomable,
    pub interactable_type: lab_core::InteractableType
}
