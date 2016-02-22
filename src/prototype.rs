//! http://gameprogrammingpatterns.com/prototype.html 

trait Monster {
    fn health(&self) -> i32;
    fn speed(&self) -> u32;
    fn clone(&self) -> Self;
    fn clone_new() -> Self; // For second style of spawner.
}

#[derive(Debug)]
struct Ghost {
    health: i32,
    speed: u32,
}

impl Ghost {
    fn new(health: i32, speed: u32) -> Ghost {
        Ghost {
            health: health,
            speed: speed,
        }
    }
}

impl Monster for Ghost {
    fn health(&self) -> i32 { self.health }
    fn speed(&self) -> u32 { self.speed }
    fn clone(&self) -> Ghost {
        Ghost {
            speed: self.speed,
            health: self.health,
        }
    }
    fn clone_new() -> Ghost {
        Ghost {
            speed: 15,
            health: 3,
        }
    }
}

struct Spawner<T: Monster> {
    prototype: T
}

impl<T: Monster> Spawner<T> {
    pub fn new(prototype: T) -> Spawner<T> {
        Spawner {
            prototype: prototype,
        }
    }
    pub fn spawn(&self) -> T {
        self.prototype.clone()
    }
}

// Second style of spawner described in Templates section.
struct SpawnerT;

impl SpawnerT {
    pub fn spawn<T: Monster>() -> T {
        T::clone_new()
    }
}

pub fn test() {
    println!("\n---------------------------");
    println!("Prototype test.\n");
    let ghost_prototype = Ghost::new(15, 3);
    let ghost_spawner = Spawner::new(ghost_prototype);
    let ghost = ghost_spawner.spawn();
    println!("Ghost1");
    println!("{:#?}", ghost);    

    // Generic style
    let ghost2 = SpawnerT::spawn::<Ghost>();
    println!("Ghost2");
    println!("{:#?}", ghost2);    
}
