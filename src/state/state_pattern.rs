//! State pattern example from http://gameprogrammingpatterns.com/state.html.
//! # NOT IMPLEMENTED YET
//! TODO We have to hold generic HeroineStates in some way in Heroine. Don't know what to do about
//! it right now.
//!
use std::cell::RefCell;
use state::{Input, Graphic};
const JUMP_VELOCITY: i32 = 10;
const MAX_CHARGE: u32 = 3;

// Heroine Herself
#[derive(Clone)]
struct Heroine {
    y_velocity: i32,
    x_velocity: i32,
    graphic: Graphic,
    state: RefCell<Box<HeroineState>>,
}

impl Heroine {
    pub fn new() -> Heroine {
        Heroine {
            state: RefCell::new(Box::new(StandingState::new())),
            y_velocity: 0,
            x_velocity: 0,
            graphic: Graphic::Stand,
        }
    }
    pub fn handle_input(&mut self, input: Input) {
        self.state.borrow_mut().clone().handle_input(self, input);
    }
    pub fn update(&mut self) {
        self.state.borrow_mut().update(&mut self);
    }
    pub fn set_state<T: 'static + HeroineState>(&mut self, state: Box<T>) {
        self.state = RefCell::new(state);
        self.state.borrow_mut().enter(&mut self);
    }
    pub fn set_y_vel(&mut self, vel: i32) {
        self.y_velocity = vel;
    }
    pub fn set_x_vel(&mut self, vel: i32) {
        self.x_velocity = vel;
    }
    pub fn set_graphic(&mut self, graphic: Graphic) {
        println!("Changing graphic to: {:?}", graphic);
        self.graphic = graphic;
    }
    pub fn super_bomb(&self) {
        println!("BOOM!");
    }
}

// Heroine State
trait HeroineState {
    fn handle_input(&mut self, heroine: &mut Heroine, input: Input);
    fn update(&mut self, heroine: &mut Heroine);
    fn enter(&mut self, heroine: &mut Heroine);
}

// Standing State
struct StandingState;

impl StandingState {
    pub fn new() -> StandingState {
        StandingState
    }
}

impl HeroineState for StandingState {
    fn enter(&mut self, heroine: &mut Heroine) {
        println!("Entering Standing State!");
        heroine.set_graphic(Graphic::Stand);
    }
    fn handle_input(&mut self, heroine: &mut Heroine, input: Input) {
        match input {
            Input::PressB => {
                heroine.set_state(Box::new(JumpState::new()));
            }
            Input::PressDown => {
                heroine.set_state(Box::new(DuckingState::new()));
            }
            _ => {}
        }
    }
    fn update(&mut self, heroine: &mut Heroine) {
        // ...
    }
}


// Ducking State
struct DuckingState {
    charge_time: u32
}

impl DuckingState {
    pub fn new() -> DuckingState {
        DuckingState {
            charge_time: 0
        }
    }
}

impl HeroineState for DuckingState {
    fn enter(&mut self, heroine: &mut Heroine) {
        heroine.set_graphic(Graphic::Duck)
    }
    fn handle_input(&mut self, heroine: &mut Heroine, input: Input) {
        match input {
            Input::ReleaseDown => {
                heroine.set_state(Box::new(StandingState::new()));
            }
            _ => {}
        }
    }
    fn update(&mut self, heroine: &mut Heroine) {
        self.charge_time += 1;
        if self.charge_time > MAX_CHARGE {
            heroine.super_bomb();
            self.charge_time = 0;
        }
    }
}


// Jumping State
pub struct JumpState;

impl JumpState {
    pub fn new() -> JumpState {
        JumpState
    }
}

impl HeroineState for JumpState {
    fn enter(&mut self, heroine: &mut Heroine) {
        println!("Entering JumpState!");
        heroine.set_graphic(Graphic::Jump);
        heroine.y_velocity = JUMP_VELOCITY;
    }
    fn handle_input(&mut self, heroine: &mut Heroine, input: Input) {
        match input {
            Input::PressDown => {
                heroine.set_state(Box::new(DivingState::new()));
            }
            _ => {}
        }
    }
    fn update(&mut self, heroine: &mut Heroine) {
        // ...
    }
}


// Diving State
pub struct DivingState;

impl DivingState {
    pub fn new() -> DivingState {
        DivingState
    }
}

impl HeroineState for DivingState {
    fn handle_input(&mut self, heroine: &mut Heroine, input: Input) {
        // ...
    }
    fn update(&mut self, heroine: &mut Heroine) {
        // ...
    }
    fn enter(&mut self, heroine: &mut Heroine) {
        println!("Entering DivingState");
        heroine.set_graphic(Graphic::Dive);
    }
}


fn test() {
    println!("\n---------------------------");
    println!("State pattern test.\n");
    let mut hero = Heroine::new();
    hero.handle_input(Input::PressDown);
    hero.update();
    hero.update();
    hero.update();
    hero.update();
    hero.handle_input(Input::ReleaseDown);
    hero.update();
    hero.handle_input(Input::PressB);
    hero.update();
    hero.handle_input(Input::PressDown);
    hero.update();
}
