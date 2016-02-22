//! http://gameprogrammingpatterns.com/flyweight.html
//!
//! Some changes to how it was presented in c++ code.
//! To hold terrain types inside world we had to wrap them in Rc. Which ofcourse gives us
//! more safety but at propably small but existent performance cost.
//!
//! Holding terrain types in Rc means that our terrain types are of type Rc<Terrain> which doesn't
//! implement Copy. That means we can't easily allocate array of fixed size with them.
//! So instead i've used Vecs with specified capacity to try to avoid realocation.

use rand::{self, Rng};
use std::rc::Rc;

// Width and height of world.
const WIDTH: usize = 5;
const HEIGHT: usize = 5;

#[derive(Debug)]
struct Terrain {
    movement_cost: i32,
    is_water: bool,
}

impl Terrain {
    pub fn new(movement_cost: i32, is_water: bool) -> Terrain {
        Terrain {
            movement_cost: movement_cost,
            is_water: is_water,
        }
    }

    pub fn is_water(&self) -> bool {
        self.is_water
    }
    pub fn get_movement_cost(&self) -> i32 {
        self.movement_cost
    }
}

#[derive(Debug)]
struct World {
    tiles: Vec<Vec<Rc<Terrain>>>,
    grass_terrain: Rc<Terrain>,
    hill_terrain: Rc<Terrain>,
    river_terrain: Rc<Terrain>,
}

impl World {
    pub fn new() -> World {
        let gt = Rc::new(Terrain::new(1, false));
        let ht = Rc::new(Terrain::new(2, false));
        let rt = Rc::new(Terrain::new(3, true));

        // Create array from vectors.
        // Specify capacity for them so we don't get realloaction for known size of the World.
        // Then as the initialization fill them with grass terrain.
        let mut tiles_x = Vec::with_capacity(WIDTH);
        for _ in 0..WIDTH {
            tiles_x.push(gt.clone())
        }
        let mut tiles: Vec<Vec<Rc<Terrain>>> = Vec::with_capacity(HEIGHT);
        for _ in 0..HEIGHT {
            tiles.push(tiles_x.clone());
        }
        World {
            grass_terrain: gt,
            hill_terrain: ht,
            river_terrain: rt,
            tiles: tiles,
        }
    }

    pub fn generate_world(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if rand::thread_rng().gen_range(0, 11) == 10 {
                    self.tiles[x][y] = self.hill_terrain.clone();
                } else {
                    self.tiles[x][y] = self.grass_terrain.clone();
                }
            }
        }
        let x: usize = rand::thread_rng().gen_range(0, WIDTH);
        // Random river
        for y in 0..HEIGHT {
            self.tiles[x][y] = self.river_terrain.clone();
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Rc<Terrain> {
        self.tiles[x][y].clone()
    }
}

pub fn test() {
    println!("---------------------------");
    println!("Flyweight test.\n");
    let mut world = World::new();
    world.generate_world();
    println!("{:#?}", world);
    let tile = world.get_tile(2, 4);
    println!("Tile at: [2][4]\nCost {}, Is it water? {}",
             tile.get_movement_cost(),
             tile.is_water());
}
