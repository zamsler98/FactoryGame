use std::collections::HashMap;

/// Simple integer tile position (origin top-left)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TilePos {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct Size2 {
    pub w: u32,
    pub h: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rotation {
    R0,
    R90,
    R180,
    R270,
}

impl Rotation {
    /// Unit tile offset a building faces: R0 = +x (right); +y is down-screen.
    /// The renderer's direction arrows and the item-transfer logic both use
    /// this, so they can never disagree.
    pub fn dir(self) -> (i32, i32) {
        match self {
            Rotation::R0 => (1, 0),
            Rotation::R90 => (0, 1),
            Rotation::R180 => (-1, 0),
            Rotation::R270 => (0, -1),
        }
    }

    /// Next rotation clockwise (R0 -> R90 -> R180 -> R270 -> R0).
    pub fn rotated_cw(self) -> Rotation {
        match self {
            Rotation::R0 => Rotation::R90,
            Rotation::R90 => Rotation::R180,
            Rotation::R180 => Rotation::R270,
            Rotation::R270 => Rotation::R0,
        }
    }
}

pub type InstanceId = u64;

#[derive(Clone, Debug)]
pub struct BuildingSpec {
    pub spec_id: u32,
    pub size: Size2,
}

#[derive(Clone, Debug)]
pub struct BuildingInstance {
    pub id: InstanceId,
    pub spec_id: u32,
    pub origin: TilePos,
    pub rotation: Rotation,
    pub size: Size2,
}

#[derive(Debug)]
pub enum PlacementError {
    OutOfBounds,
    Occupied,
}

pub struct TileGrid {
    pub width: usize,
    pub height: usize,
    tiles: Vec<Option<InstanceId>>,
    pub instances: HashMap<InstanceId, BuildingInstance>,
    next_id: InstanceId,
}

impl TileGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![None; width.saturating_mul(height)];
        Self {
            width,
            height,
            tiles,
            instances: HashMap::new(),
            next_id: 1,
        }
    }

    fn tile_index(&self, pos: TilePos) -> Option<usize> {
        if pos.x < 0 || pos.y < 0 {
            return None;
        }
        let x = pos.x as usize;
        let y = pos.y as usize;
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(y * self.width + x)
    }

    pub fn tile_occupant(&self, pos: TilePos) -> Option<InstanceId> {
        self.tile_index(pos).and_then(|i| self.tiles[i])
    }

    fn rotated_size(size: Size2, rot: Rotation) -> Size2 {
        match rot {
            Rotation::R0 | Rotation::R180 => size,
            Rotation::R90 | Rotation::R270 => Size2 {
                w: size.h,
                h: size.w,
            },
        }
    }

    fn footprint_tiles(size: Size2, origin: TilePos, rot: Rotation) -> Vec<TilePos> {
        let rs = Self::rotated_size(size, rot);
        let mut v = Vec::with_capacity((rs.w * rs.h) as usize);
        for dy in 0..(rs.h as i32) {
            for dx in 0..(rs.w as i32) {
                v.push(TilePos {
                    x: origin.x + dx,
                    y: origin.y + dy,
                });
            }
        }
        v
    }

    pub fn can_place(&self, spec: &BuildingSpec, origin: TilePos, rot: Rotation) -> bool {
        let tiles = Self::footprint_tiles(spec.size, origin, rot);
        for t in tiles {
            match self.tile_index(t) {
                Some(idx) => {
                    if self.tiles[idx].is_some() {
                        return false;
                    }
                }
                None => return false,
            }
        }
        true
    }

    pub fn place(
        &mut self,
        spec: &BuildingSpec,
        origin: TilePos,
        rot: Rotation,
    ) -> Result<InstanceId, PlacementError> {
        if !self.can_place(spec, origin, rot) {
            // determine if out of bounds vs occupied
            let tiles = Self::footprint_tiles(spec.size, origin, rot);
            for t in tiles {
                if self.tile_index(t).is_none() {
                    return Err(PlacementError::OutOfBounds);
                }
                if let Some(idx) = self.tile_index(t) {
                    if self.tiles[idx].is_some() {
                        return Err(PlacementError::Occupied);
                    }
                }
            }
            return Err(PlacementError::Occupied);
        }
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        let instance = BuildingInstance {
            id,
            spec_id: spec.spec_id,
            origin,
            rotation: rot,
            size: spec.size,
        };
        let tiles = Self::footprint_tiles(spec.size, origin, rot);
        for t in tiles {
            if let Some(idx) = self.tile_index(t) {
                self.tiles[idx] = Some(id);
            }
        }
        self.instances.insert(id, instance);
        Ok(id)
    }

    pub fn remove(&mut self, id: InstanceId) -> Option<BuildingInstance> {
        let inst = self.instances.remove(&id)?;
        // clear tiles occupied by this instance
        let tiles = Self::footprint_tiles(inst.size, inst.origin, inst.rotation);
        for t in tiles {
            if let Some(idx) = self.tile_index(t) {
                if self.tiles[idx] == Some(id) {
                    self.tiles[idx] = None;
                }
            }
        }
        Some(inst)
    }
}
