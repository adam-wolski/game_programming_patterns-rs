//! State pattern
//! from http://gameprogrammingpatterns.com/state.html.

use std::cell::RefCell;
use std::rc::Rc;

use state::{Input, Graphic};
const JUMP_VELOCITY: i32 = 10;
const MAX_CHARGE: u32 = 3;

/// State wraped in a way that enables it's storage and mutable access in Heroine.
type WrapedState = Rc<RefCell<Box<HeroineState>>>;

/// Macro for creating Reference counter of Reference Cell with box of the state in it.
macro_rules! wrap_state {
    ($state:expr) => {
        Rc::new(RefCell::new(Box::new($state)))
    }
}

/// Heroine Herself
pub struct Heroine {
    y_velocity: i32,
    x_velocity: i32,
    graphic: Graphic,
    state: WrapedState,
}


impl Heroine {
    pub fn new() -> Heroine {
        Heroine {
            state: wrap_state!(StandingState::new()),
            y_velocity: 0,
            x_velocity: 0,
            graphic: Graphic::Stand,
        }
    }

    pub fn handle_input(&mut self, input: Input) {
        let state = self.state.clone();
        state.borrow_mut().handle_input(self, input);
    }

    pub fn update(&mut self) {
        let state = self.state.clone();
        state.borrow_mut().update(self);
    }

    pub fn set_state(&mut self, state: Rc<RefCell<Box<HeroineState>>>) {
        state.borrow_mut().enter(self);
        self.state = state;
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

impl Default for Heroine {
    fn default() -> Heroine {
        Heroine {
            state: wrap_state!(StandingState::new()),
            y_velocity: 0,
            x_velocity: 0,
            graphic: Graphic::Stand,
        }
    }
}

pub trait HeroineState {
    fn state_name(&self) -> String;
    fn handle_input(&mut self, heroine: &mut Heroine, input: Input);
    fn update(&mut self, heroine: &mut Heroine);
    fn enter(&mut self, heroine: &mut Heroine);
}

#[derive(Default)]
struct StandingState {
    name: String,
}

impl StandingState {
    pub fn new() -> StandingState {
        StandingState { name: "StandingState".to_owned() }
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
                heroine.set_state(wrap_state!(JumpState::new()));
            }
            Input::PressDown => {
                heroine.set_state(wrap_state!(DuckingState::new()));
            }
            _ => {}
        }
    }
    fn update(&mut self, _heroine: &mut Heroine) {
        // ...
    }
    fn state_name(&self) -> String {
        self.name.clone()
    }
}


// Ducking State
#[derive(Default)]
struct DuckingState {
    name: String,
    charge_time: u32,
}

impl DuckingState {
    pub fn new() -> DuckingState {
        DuckingState {
            name: "DuckingState".to_owned(),
            charge_time: 0,
        }
    }
}

impl HeroineState for DuckingState {
    fn state_name(&self) -> String {
        self.name.clone()
    }
    fn enter(&mut self, heroine: &mut Heroine) {
        heroine.set_graphic(Graphic::Duck)
    }
    fn handle_input(&mut self, heroine: &mut Heroine, input: Input) {
        if let Input::ReleaseDown = input {
            heroine.set_state(wrap_state!(StandingState::new()));
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
#[derive(Default)]
pub struct JumpState {
    name: String,
}

impl JumpState {
    pub fn new() -> JumpState {
        JumpState { name: "JumpState".to_owned() }
    }
}

impl HeroineState for JumpState {
    fn state_name(&self) -> String {
        self.name.clone()
    }
    fn enter(&mut self, heroine: &mut Heroine) {
        println!("Entering JumpState!");
        heroine.set_graphic(Graphic::Jump);
        heroine.y_velocity = JUMP_VELOCITY;
    }
    fn handle_input(&mut self, heroine: &mut Heroine, input: Input) {
        if let Input::PressDown = input {
            heroine.set_state(wrap_state!(DivingState::new()));
        }
    }
    fn update(&mut self, _heroine: &mut Heroine) {
        // ...
    }
}


// Diving State
#[derive(Default)]
pub struct DivingState {
    name: String,
}

impl DivingState {
    pub fn new() -> DivingState {
        DivingState { name: "DivingState".to_owned() }
    }
}

impl HeroineState for DivingState {
    fn state_name(&self) -> String {
        self.name.clone()
    }
    fn handle_input(&mut self, _heroine: &mut Heroine, _input: Input) {
        // ...
    }
    fn update(&mut self, _heroine: &mut Heroine) {
        // ...
    }
    fn enter(&mut self, heroine: &mut Heroine) {
        println!("Entering DivingState");
        heroine.set_graphic(Graphic::Dive);
    }
}


#[cfg(test)]
mod tests {
    use super::{Heroine, HeroineState};
    use super::super::Input;

    #[test]
    fn state_pattern() {
        println!("\n---------------------------");
        println!("State pattern test.\n");
        let mut hero = Heroine::new();
        assert!(hero.state.borrow().state_name() == "StandingState");
        hero.handle_input(Input::PressDown);
        hero.update();
        hero.update();
        assert!(hero.state.borrow().state_name() == "DuckingState");
        hero.update();
        hero.update();
        assert!(hero.state.borrow().state_name() == "DuckingState");
        hero.handle_input(Input::ReleaseDown);
        assert!(hero.state.borrow().state_name() == "StandingState");
        hero.update();
        assert!(hero.state.borrow().state_name() == "StandingState");
        hero.handle_input(Input::PressB);
        assert!(hero.state.borrow().state_name() == "JumpState");
        hero.update();
        assert!(hero.state.borrow().state_name() == "JumpState");
        hero.handle_input(Input::PressDown);
        assert!(hero.state.borrow().state_name() == "DivingState");
        hero.update();
        assert!(hero.state.borrow().state_name() == "DivingState");
    }
}
