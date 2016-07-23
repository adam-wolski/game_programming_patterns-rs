//! Flyweight Pattern
//! http://gameprogrammingpatterns.com/flyweight.html


use rand::{self, Rng};
use std::rc::Rc;


/// Example World Width 
const WIDTH: usize = 5;
/// Example World Height 
const HEIGHT: usize = 5;


/// Terrain type that will be referenced around using the pattern.
#[derive(Debug, Default)]
pub struct Terrain {
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


/// Our game world structure containing all the tiles with their associated terrain type.
/// That's basically whole pattern. Make terrain type only once in memory and then reference it.
#[derive(Debug, Default)]
pub struct World {
    pub tiles: Vec<Vec<Rc<Terrain>>>,
    pub grass_terrain: Rc<Terrain>,
    pub hill_terrain: Rc<Terrain>,
    pub river_terrain: Rc<Terrain>,
}

impl World {
    pub fn new() -> World {
        let gt = Rc::new(Terrain::new(1, false));
        let ht = Rc::new(Terrain::new(2, false));
        let rt = Rc::new(Terrain::new(3, true));

        // Create array from vectors.
        // Specify capacity for them so we don't get realloaction for known size of the World.
        let mut tiles_x = Vec::with_capacity(WIDTH);

        // Then as the initialization fill them with grass terrain.
        for _ in 0..WIDTH {
            tiles_x.push(gt.clone())
        }
        // and again.
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

        // Random hills and grass.
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if rand::thread_rng().gen_range(0, 11) == 10 {
                    self.tiles[x][y] = self.hill_terrain.clone();
                } else {
                    self.tiles[x][y] = self.grass_terrain.clone();
                }
            }
        }

        // Random river.
        let x: usize = rand::thread_rng().gen_range(0, WIDTH);
        for y in 0..HEIGHT {
            self.tiles[x][y] = self.river_terrain.clone();
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Rc<Terrain> {
        self.tiles[x][y].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::World;

    #[test]
    pub fn flyweight() {
        let mut world = World::new();
        world.generate_world();
        let tile = world.get_tile(2, 4);
        println!("{:#?}", tile);
    }
}
