//! game_core: pure game state and simple ECS-ish world.
//! This crate must not depend on Macroquad or any platform APIs.
//! It contains the world, entities, and deterministic update functions.

use std::fmt;

pub type EntityId = u32;

mod grid;
pub use grid::*;

#[derive(Clone, Debug)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EntityType {
    Player,
    Enemy,
}

#[derive(Clone, Debug)]
pub struct Entity {
    pub id: EntityId,
    pub ty: EntityType,
    pub transform: Transform,
    pub velocity: Velocity,
    pub radius: f32,
}

/// Lightweight item type used for conveyors/miner/smelter items.
#[derive(Clone, Debug)]
pub struct Item {
    pub id: u64,
    pub transform: Transform,
    pub velocity: Velocity,
    pub radius: f32,
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Entity {{ id: {}, ty: {:?}, pos: ({:.1},{:.1}), vel: ({:.1},{:.1}) }}",
            self.id,
            self.ty,
            self.transform.x,
            self.transform.y,
            self.velocity.vx,
            self.velocity.vy
        )
    }
}

/// Minimal world container with deterministic update (physics integration).
pub struct World {
    pub entities: Vec<Entity>, // intentionally public for iterating/drawing
    pub items: Vec<Item>,      // moving items on conveyors
    pub tile_grid: TileGrid,
    next_id: EntityId,
    next_item_id: u64,
}

impl World {
    /// Create an empty world.
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            items: Vec::new(),
            tile_grid: TileGrid::new(1000, 1000),
            next_id: 1,
            next_item_id: 1,
        }
    }

    fn alloc_id(&mut self) -> EntityId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn alloc_item_id(&mut self) -> u64 {
        let id = self.next_item_id;
        self.next_item_id = self.next_item_id.saturating_add(1);
        id
    }

    /// Spawn a player entity at position.
    pub fn spawn_player(&mut self, x: f32, y: f32) {
        let e = Entity {
            id: self.alloc_id(),
            ty: EntityType::Player,
            transform: Transform { x, y },
            velocity: Velocity { vx: 0.0, vy: 0.0 },
            radius: 16.0,
        };
        self.entities.push(e);
    }

    /// Spawn a simple enemy.
    pub fn spawn_enemy(&mut self, x: f32, y: f32) {
        let e = Entity {
            id: self.alloc_id(),
            ty: EntityType::Enemy,
            transform: Transform { x, y },
            velocity: Velocity { vx: 0.0, vy: 0.0 },
            radius: 12.0,
        };
        self.entities.push(e);
    }

    /// Simple physics integration: position += velocity * dt.
    /// Also perform basic world bounds clamping (optional).
    pub fn update_physics(&mut self, dt: f32) {
        for e in &mut self.entities {
            e.transform.x += e.velocity.vx * dt;
            e.transform.y += e.velocity.vy * dt;

            // keep things inside a simple visible world rectangle (0..2000)
            if e.transform.x < -1000.0 {
                e.transform.x = -1000.0;
                e.velocity.vx = 0.0;
            }
            if e.transform.x > 1000.0 {
                e.transform.x = 1000.0;
                e.velocity.vx = 0.0;
            }
            if e.transform.y < -1000.0 {
                e.transform.y = -1000.0;
                e.velocity.vy = 0.0;
            }
            if e.transform.y > 1000.0 {
                e.transform.y = 1000.0;
                e.velocity.vy = 0.0;
            }
        }

        // update items as well
        for it in &mut self.items {
            it.transform.x += it.velocity.vx * dt;
            it.transform.y += it.velocity.vy * dt;

            if it.transform.x < -1000.0 {
                it.transform.x = -1000.0;
                it.velocity.vx = 0.0;
            }
            if it.transform.x > 1000.0 {
                it.transform.x = 1000.0;
                it.velocity.vx = 0.0;
            }
            if it.transform.y < -1000.0 {
                it.transform.y = -1000.0;
                it.velocity.vy = 0.0;
            }
            if it.transform.y > 1000.0 {
                it.transform.y = 1000.0;
                it.velocity.vy = 0.0;
            }
        }
    }

    /// Helper: find mutable reference to the player entity (first occurrence).
    pub fn find_player_mut(&mut self) -> Option<&mut Entity> {
        self.entities
            .iter_mut()
            .find(|e| e.ty == EntityType::Player)
    }

    /// Spawn a simple item (used by miners). Returns the item id.
    pub fn spawn_item(&mut self, x: f32, y: f32, vx: f32, vy: f32, radius: f32) -> u64 {
        let id = self.alloc_item_id();
        let it = Item {
            id,
            transform: Transform { x, y },
            velocity: Velocity { vx, vy },
            radius,
        };
        self.items.push(it);
        id
    }

    /// Remove item by id, returning it if present.
    pub fn remove_item(&mut self, id: u64) -> Option<Item> {
        if let Some(idx) = self.items.iter().position(|it| it.id == id) {
            Some(self.items.remove(idx))
        } else {
            None
        }
    }

    /// Find mutable reference to item by id
    pub fn find_item_mut(&mut self, id: u64) -> Option<&mut Item> {
        self.items.iter_mut().find(|it| it.id == id)
    }

    /// Helper: find immutable reference to the player entity (first occurrence).
    pub fn find_player(&self) -> Option<&Entity> {
        self.entities.iter().find(|e| e.ty == EntityType::Player)
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_and_update() {
        let mut world = World::new();
        world.spawn_player(0.0, 0.0);
        {
            let p = world.find_player_mut().unwrap();
            p.velocity.vx = 10.0;
            p.velocity.vy = 0.0;
        }
        world.update_physics(1.0);
        let p = world.find_player().unwrap();
        assert_eq!(p.transform.x, 10.0);
    }
}
