use bevy::{prelude::*};
use lab_core::{Zoomable, stage};
use std::collections::BTreeMap;
use std::{fmt::Debug, collections::btree_map::{Values, Keys}};
use lab_sprites::{TileAnimation, SpriteInfo};
use lab_entities::prelude::*;
mod systems;

pub mod settings {
    pub const TILE_SIZE : f32 = 16.;
    pub const WORLD_TILE_SIZE : f32 = 16.;
    pub const PLAYER_SPEED : f32 = 48.;
}

/// Plugin that will setup all of the rules of the world.
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(systems::npc_move_system.system())
            .add_resource(TilePalette::default())
            //.add_system(systems::add_world_sprites_system.system())
            //.add_system(systems::add_interaction_sprites_system.system())
            .add_system(systems::save_world_system.thread_local_system())
            .add_system(systems::tile_interaction_system.system())            
            .add_system(systems::sprite_despawn_system.system())
            .add_system(systems::static_text_system.system())
            .add_system(systems::object_interaction_system.system())
            .add_system_to_stage(stage::PROCESSING, systems::zoom_system.system())
            .add_system_to_stage(stage::POST_UPDATE, systems::camera_tracking_system.system());
    }
}


#[derive(Default, Clone, Debug)]
pub struct TilePalette {
    pub components: BTreeMap<String, TileComponents>
}

impl TilePalette {
    pub fn get_interaction(&self, name: String) -> Option<Interaction> {
        match self.components.get(&name) {
            Some(comps) => Some(comps.interaction),
            None => None
        }
    }

    pub fn tile_names(&self) -> Keys<'_, String, TileComponents>{
        self.components.keys()
    }

    pub fn iter(&self) -> Values<'_, String, TileComponents> {
        self.components.values()
    }

    pub fn tile_categories(&self) -> Vec<&str> {
        let mut categories : Vec<&str> = self.components.values().map(|m| &m.sprite.category[..]).collect();
        
        categories.sort();
        categories.dedup();
        
        categories
    }

    pub fn tiles_in_category(&self, category : &str) -> Vec<&TileComponents> {
        self.components.values().filter(|p| p.sprite.category == category).collect()        
    }

    pub fn update( &mut self, comp : &TileComponents) {

        if let Some(tc) = self.components.get_mut(&comp.sprite.name) {
           *tc = comp.clone();
        } else {
            self.components.insert(comp.sprite.name.clone(), comp.clone());
        }


    }
}


pub enum InteractionResult {
    ChangeTile(TileAttributes),
    Damage(u32),
    ChangeSprite(SpriteInfo),
    Move(Location),
    PickUp(lab_entities::objs::Item),
    Block,
    None
}
#[derive(Copy, Clone)]
pub struct Interaction {
    pub description: &'static str,
    pub call : fn (InteractionContext) -> InteractionResult
}

impl Debug for Interaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interaction")
        .field("description", &self.description)
        .finish()
    }
}


/// Attributes for a tile. These are meant to be changed by the player or interactions.

#[derive(Default, Clone, Copy, Properties, Debug)]
pub struct TileAttributes {
    pub hit_points: u32,
    pub hardness: f32, 
    pub sprite_idx: Option<u32>
}

#[derive(Bundle, Clone, Debug)]
pub struct TileComponents {
    pub hardness: Hardness,
    pub tile_type: TileType,
    pub location: Location,
    pub visible: Visible,
    pub interaction: Interaction,
    pub sprite: SpriteInfo,
    pub animation: TileAnimation,
    pub tile_attributes: TileAttributes,
    pub zoomable: Zoomable
}

impl TileComponents {
    pub fn hardness_from_tile(tile_type: TileType) -> Hardness {
        match tile_type {
            TileType::Immutable => Hardness(999.), 
            TileType::Breakable(h ) =>  h,
            _ => Hardness(0.),
        }
    }
}

impl Default for TileComponents {
    fn default() -> Self {
        TileComponents {
            hardness: Hardness(0.),
            tile_type: TileType::Floor,
            location: Location::default(),
            visible: Visible,
            interaction: Interaction { description: "pass interaction", call: |ctx| {
                match ctx.tile_attributes  { 
                    Some(tile_attributes) =>{
                        if tile_attributes.hardness != 0. {
                            InteractionResult::Block
                        } else {
                            InteractionResult::None 
                        }
                    },
                    None => InteractionResult::None 
                }                
            } },
            tile_attributes: TileAttributes { hit_points: 0, hardness: 0.0, sprite_idx: None },
            sprite: SpriteInfo::default(),
            animation: TileAnimation::default(),
            zoomable: Zoomable
        }
    }
}

#[derive(Default)]
pub struct InteractionContext <'a> {
    pub inventory: Option<&'a mut crate::player::Inventory>,
    pub player: Option<Entity>,
    pub player_location: Option<Location>,
    pub interaction_location: Option<Location>,
    pub sprite_info: Option<&'a SpriteInfo>,
    pub tile_attributes: Option<TileAttributes>,
    pub tile_palette: Option<&'a TilePalette>
}


#[derive(Copy, Clone, Debug)]
pub struct Interactable;
#[derive(Bundle, Copy, Clone, Debug)]
pub struct InteractableComponents {
    pub interactable: Interactable,
    pub location: Location,
    pub visible: Visible,
    pub interaction: Interaction
}

impl InteractableComponents {
   
}

impl Default for InteractableComponents {
    fn default() -> Self {
        InteractableComponents {
            location: Location::default(),
            visible: Visible,
            interaction: Interaction { description: "pass interaction", call: |ctx| {
                match ctx.tile_attributes  { 
                    Some(tile_attributes) =>{
                        if tile_attributes.hardness != 0. {
                            InteractionResult::Block
                        } else {
                            InteractionResult::None 
                        }
                    },
                    None => InteractionResult::None 
                }                
            } },
            interactable: Interactable
        }
    }
}