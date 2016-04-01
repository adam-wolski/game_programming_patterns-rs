//! Component Pattern
//! http://gameprogrammingpatterns.com/component.html
//! There are many ways to implement component pattern. One shown in gameprogrammingpatterns
//! requires circlical reference which is not really loved by Rust.
//!
//! One showed in here is going with intention that no logic is stored in Component and it's a job
//! for another system to perform necessary operations.
//! Based on:
//! http://www.gamedev.net/page/resources/_/technical/game-programming/implementing-component-entity-systems-r3382


// ================================================================================================
// Components
// ================================================================================================

enum_from_primitive! {
    enum Component {
        Empty        = 0,
        Velocity     = 1 << 1,
        Displacement = 1 << 2,
        Appearance   = 1 << 3,
    }
}

#[derive(Default, Debug, Clone)]
pub struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Default, Debug, Clone)]
pub struct Displacement {
    x: f32,
    y: f32,
}

#[derive(Default, Debug, Clone)]
pub struct Appearance {
    name: String,
}


// ================================================================================================
// Entity and World
// ================================================================================================

pub type EntityId = usize;

pub struct World {
    pub masks: Vec<u32>,
    pub displacements: Vec<Displacement>,
    pub velocitys: Vec<Velocity>,
    pub appearances: Vec<Appearance>,
    pub entity_count: usize,
}

impl World {
    pub fn new() -> World {
        let entity_count: usize = 100; // Starting amount of entities.
        let masks: Vec<u32> = vec!(0; entity_count);
        let displacements: Vec<Displacement> = vec!(Displacement::default(); entity_count);
        let velocitys: Vec<Velocity> = vec!(Velocity::default(); entity_count);
        let appearances: Vec<Appearance> = vec!(Appearance::default(); entity_count);
        World {
            masks: masks,
            entity_count: entity_count,
            displacements: displacements,
            velocitys: velocitys,
            appearances: appearances,
        }
    }

    fn create_entity(&self) -> Option<EntityId> {
        // XXX: Don't clone in here.
        for (id, mask) in self.masks.clone().into_iter().enumerate() {
            if mask == Component::Empty as u32 {
                println!("Found empty id: {}", id);
                return Some(id);
            }
        }
        println!("No empty ids");
        None
    }

    pub fn destroy_entity(&mut self, id: usize) {
        self.masks[id] = Component::Empty as u32;
        println!("Destroyed entity with id: {}", id);
    }

    pub fn create_tree(&mut self, x: f32, y: f32) -> EntityId {
        let id = self.create_entity().unwrap();

        self.masks[id] = Component::Displacement as u32 | Component::Appearance as u32;

        self.displacements[id].x = x;
        self.displacements[id].y = y;
        self.appearances[id].name = "Tree".to_owned();
        id
    }

    pub fn create_box(&mut self, x: f32, y: f32) -> EntityId {
        let id = self.create_entity().unwrap();

        self.masks[id] = Component::Displacement as u32 | Component::Appearance as u32 |
                         Component::Velocity as u32;

        self.displacements[id].x = x;
        self.displacements[id].y = y;
        self.velocitys[id].x = 1.0;
        self.velocitys[id].y = 2.0;
        self.appearances[id].name = "Box".to_owned();
        id
    }

    pub fn create_ghost(&mut self, x: f32, y: f32) -> EntityId {
        let id = self.create_entity().unwrap();

        self.masks[id] = Component::Displacement as u32 | Component::Velocity as u32;

        self.displacements[id].x = x;
        self.displacements[id].y = y;
        self.velocitys[id].x = 3.0;
        self.velocitys[id].y = 2.0;
        id
    }

    /// Get vector of entities with specified mask.
    pub fn get_with_mask(&self, mask: u32) -> Vec<EntityId> {
        // XXX: Find way to don't clone here.
        self.masks
            .clone()
            .into_iter()
            .enumerate()
            .filter(|&(_id, msk)| msk & mask == mask)
            .map(|(id, _msk)| id)
            .collect()
    }
}

impl Default for World {
    fn default() -> World {
        let entity_count: usize = 100; // Starting amount of entities.
        let masks: Vec<u32> = vec!(0; entity_count);
        let displacements: Vec<Displacement> = vec!(Displacement::default(); entity_count);
        let velocitys: Vec<Velocity> = vec!(Velocity::default(); entity_count);
        let appearances: Vec<Appearance> = vec!(Appearance::default(); entity_count);
        World {
            masks: masks,
            entity_count: entity_count,
            displacements: displacements,
            velocitys: velocitys,
            appearances: appearances,
        }
    }
}

// ================================================================================================
// Systems
// ================================================================================================

pub fn movement_system(world: &mut World) {
    let movement_mask: u32 = Component::Displacement as u32 | Component::Velocity as u32;
    for entity in world.get_with_mask(movement_mask) {
        println!("Moving entity {}", entity);

        let disp = &mut world.displacements[entity];
        let vel = &mut world.velocitys[entity];

        vel.y -= vel.y;
        disp.x += vel.x;
        disp.y += vel.y;
    }
}

pub fn render_system(world: &mut World) {
    let render_mask: u32 = Component::Displacement as u32 | Component::Appearance as u32;
    for entity in world.get_with_mask(render_mask) {
        let disp = &world.displacements[entity];
        let app = &world.appearances[entity];
        println!("Drawing {} at {:?}", app.name, disp);
    }
}
