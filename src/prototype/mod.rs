//! Prototype Pattern
//! http://gameprogrammingpatterns.com/prototype.html

pub trait Monster {
    fn health(&self) -> i32;
    fn speed(&self) -> u32;
    /// Clone itself with all the members values.
    fn clone(&self) -> Self;
    /// Used in template style spawner. 
    /// This one creates new Monster with defaults specified in that method.
    fn clone_new() -> Self;
}

#[derive(Debug)]
pub struct Ghost {
    health: i32,
    speed: u32,
}

impl Ghost {
    pub fn new(health: i32, speed: u32) -> Ghost {
        Ghost {
            health: health,
            speed: speed,
        }
    }
}

impl Monster for Ghost {
    fn health(&self) -> i32 {
        self.health
    }
    fn speed(&self) -> u32 {
        self.speed
    }
    fn clone(&self) -> Ghost {
        Ghost {
            speed: self.speed,
            health: self.health,
        }
    }
    fn clone_new() -> Ghost {
        Ghost {
            speed: 2,
            health: 8,
        }
    }
}

pub struct Spawner<T: Monster> {
    prototype: T,
}

impl<T: Monster> Spawner<T> {
    pub fn new(prototype: T) -> Spawner<T> {
        Spawner { prototype: prototype }
    }
    pub fn spawn(&self) -> T {
        self.prototype.clone()
    }
}

// Second style of spawner described in Templates section.
pub struct SpawnerT;

impl SpawnerT {
    pub fn spawn<T: Monster>() -> T {
        T::clone_new()
    }
}


#[cfg(test)]
mod tests {
    use super::{Ghost, Spawner, SpawnerT, Monster};

    #[test]
    pub fn prototype() {
        let ghost_prototype = Ghost::new(15, 3);
        let ghost_spawner = Spawner::new(ghost_prototype);
        let ghost = ghost_spawner.spawn();
        assert!(ghost.health() == 15);
        assert!(ghost.speed() == 3);

        // Using generic spawner.
        let ghost2 = SpawnerT::spawn::<Ghost>();
        assert!(ghost2.health() == 8);
        assert!(ghost2.speed() == 2);
    }
}
