//! Game Programming Patterns implemented in Rust.
#![feature(plugin)]

#![plugin(clippy)]

extern crate rand;
#[macro_use]
extern crate enum_primitive;
extern crate num;

pub mod command;
pub mod flyweight;
pub mod observer;
pub mod prototype;
pub mod state;
pub mod double_buffer;
pub mod bytecode;
pub mod type_object;

// Import traits.
use observer::{Observer, Subject};
use type_object::Monster;

use std::cell::RefCell;
use std::rc::Rc;

pub fn main() {
    // Let's use all the modules so we don't get "is never used" warnings.
    // Those are the same as in the test functions.


    // ============================================================================================
    // Design Patterns
    // ============================================================================================

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
    assert!(achievements.borrow().hero_fallen);
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
    println!("\n---------------------------");
    println!("finite state machine test.\n");
    let mut hero = state::finite_state_machine::Heroine::new();
    hero.handle_input(state::Input::PressDown);
    hero.update();
    hero.update();
    hero.update();
    hero.update();
    hero.handle_input(state::Input::ReleaseDown);
    hero.update();
    hero.handle_input(state::Input::PressB);
    hero.update();
    hero.handle_input(state::Input::PressDown);
    hero.update();

    // ## State pattern
    println!("\n---------------------------");
    println!("State pattern test.\n");
    let mut hero = state::state_pattern::Heroine::new();
    hero.handle_input(state::Input::PressDown);
    hero.update();
    hero.update();
    hero.update();
    hero.update();
    hero.handle_input(state::Input::ReleaseDown);
    hero.update();
    hero.handle_input(state::Input::PressB);
    hero.update();
    hero.handle_input(state::Input::PressDown);
    hero.update();

    // ============================================================================================
    // Sequence Patterns
    // ============================================================================================

    // # Double Buffer
    println!("\n---------------------------");
    println!("Double Buffer pattern test.\n");
    let mut scene = double_buffer::Scene::new();
    scene.draw();

    // # Game Loop
    // As we don't want to loop in here game loop sits safely in it's own module.

    // ============================================================================================
    // Behavioral Patterns
    // ============================================================================================

    // # Bytecode
    println!("\n---------------------------");
    println!("Bytecode Pattern test.\n");
    let mut bytecode: Vec<u8> = Vec::new();
    // set_health(0, get_health(0) + (get_agility(0) + get_wisdom(0)) / 2);
    bytecode.push(1);  // Literal                       []
    bytecode.push(0);  // Wizard index                  [0]
    bytecode.push(1);  // Literal
    bytecode.push(0);  // Wizard index                  [0, 0]
    bytecode.push(7);  // Get health                    [0, 5]
    bytecode.push(1);  // Literal
    bytecode.push(0);  // Wizard index                  [0, 5, 0]
    bytecode.push(8);  // Get Agility                   [0, 5, 8]
    bytecode.push(1);  // Literal
    bytecode.push(0);  // Wizard index                  [0, 5, 8, 0]
    bytecode.push(9);  // Get Wisdom                    [0, 5, 8, 12]
    bytecode.push(10); // Add Agility and wisdom        [0, 5, 20]
    bytecode.push(1);  // Literal
    bytecode.push(2);  // Divisor                       [0, 5, 20, 2]
    bytecode.push(11); // Divide                        [0, 5, 10]
    bytecode.push(10); // Add average to current health [0, 15]
    bytecode.push(2);  // Set Health                    []
    let mut vm = bytecode::VM::new(bytecode);
    vm.interpret();

    // # Type Object
    println!("\n---------------------------");
    println!("Type Object Pattern test.\n");
    let parent = type_object::Breed::new(None, 15, "ARGH!".to_owned());
    let child = type_object::Breed::new(Some(&parent), 0, "".to_owned());
    child.attack();
}
