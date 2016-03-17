//! Game Programming Patterns implemented in Rust.
#![feature(plugin)]

#![plugin(clippy)]

extern crate rand;

pub mod command;
pub mod flyweight;
pub mod observer;
pub mod prototype;
pub mod state;

// Import traits.
use observer::{Observer, Subject};

use std::cell::RefCell;
use std::rc::Rc;

pub fn main() {
    // Let's use all the modules so we don't get "is never used" warnings.
    // Those are the same as in the test functions.
    //
    // # Command
    println!("\n---------------------------");
    println!("Command test.\n");
    let mut unit = command::Player { x: 0, y: 0 };
    let cmd = command::make_move_unit_cmd(5, 10);
    cmd(&mut unit);
    println!("{}", unit.x);
    println!("{}", unit.y);

    // # Flyweight
    println!("\n---------------------------");
    println!("Flyweight test.\n");
    let mut world = flyweight::World::new();
    world.generate_world();
    let tile = world.get_tile(2, 4);
    println!("Tile at: [2][4]\nCost {}, Is it water? {}",
             tile.get_movement_cost(),
             tile.is_water());
    println!("Default: {:#?}", flyweight::World::default());

    // # Observer
    println!("\n---------------------------");
    println!("Observer test.\n");
    let hero = observer::Hero::new();
    let achievements = Rc::new(RefCell::new(observer::Achievements::new()));
    let a_observer = observer::AchievementObserver::new(achievements.clone());
    let a_observer = Rc::new(RefCell::new(Box::new(a_observer) as Box<Observer>));
    let mut physics = observer::Physics::new();
    physics.fall_event().add_observer(a_observer);
    physics.update_entity(&hero);
    assert!(achievements.borrow().hero_fallen == true);
    println!("Hero has fallen!");

    // # Prototype
    println!("\n---------------------------");
    println!("Prototype test.\n");
    let ghost_prototype = prototype::Ghost::new(15, 3);
    let ghost_spawner = prototype::Spawner::new(ghost_prototype);
    let ghost = ghost_spawner.spawn();
    println!("Ghost1");
    println!("{:#?}", ghost);
    // Using generic spawner.
    let ghost2 = prototype::SpawnerT::spawn::<prototype::Ghost>();
    println!("Ghost2");
    println!("{:#?}", ghost2);

    // # State
    // ## Finite State Machine
}
