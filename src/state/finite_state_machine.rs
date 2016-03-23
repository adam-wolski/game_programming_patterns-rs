//! Finite State Machine
//! First example from http://gameprogrammingpatterns.com/state.html
//! Using standard switch/match techniques.

use state::{Input, Graphic};

const JUMP_VELOCITY: i32 = 10;
const MAX_CHARGE: u32 = 3;

#[derive(Debug, PartialEq)]
pub enum State {
    Jumping,
    Standing,
    Ducking,
    Diving,
}

impl Default for State {
    fn default() -> State {
        State::Standing
    }
}

#[derive(Debug, Default)]
pub struct Heroine {
    pub state: State,
    pub graphic: Graphic,
    pub y_velocity: i32,
    pub charge_time: u32,
}

impl Heroine {
    pub fn new() -> Heroine {
        Heroine {
            state: State::Standing,
            graphic: Graphic::Stand,
            charge_time: 0,
            y_velocity: 0,
        }
    }

    fn set_graphics(&mut self, graphic: Graphic) {
        println!("Setting graphic to: {:?}", graphic);
        self.graphic = graphic;
    }

    fn super_bomb(&mut self) {
        println!("BOOM!");
        self.charge_time = 0;
    }

    pub fn update(&mut self) {
        if let State::Ducking = self.state {
            println!("Ducking");
            self.charge_time += 1;
            if self.charge_time > MAX_CHARGE {
                self.super_bomb();
            }
        }
    }

    pub fn handle_input(&mut self, input: Input) {
        match self.state {
            State::Standing => {
                match input {
                    Input::PressB => {
                        self.state = State::Jumping;
                        println!("Changing state to Jumping");
                        self.y_velocity = JUMP_VELOCITY;
                        self.set_graphics(Graphic::Jump);
                    }
                    Input::PressDown => {
                        self.state = State::Ducking;
                        println!("Changing state to Ducking");
                        self.charge_time = 0; //Reset timer when ducking
                        self.set_graphics(Graphic::Duck);
                    }
                    _ => {}
                }
            }
            State::Jumping => {
                if let Input::PressDown = input {
                    self.state = State::Diving;
                    println!("Changing state to Diving");
                    self.set_graphics(Graphic::Dive);
                }
            }
            State::Ducking => {
                if let Input::ReleaseDown = input {
                    self.state = State::Standing;
                    println!("Changing state to Standing");
                    self.set_graphics(Graphic::Stand);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Heroine, State};
    use super::super::Input;

    #[test]
    pub fn finite_state_machine() {
        let mut hero = Heroine::new();
        assert!(hero.state == State::Standing);
        hero.handle_input(Input::PressDown);
        assert!(hero.state == State::Ducking);
        hero.update();
        assert!(hero.state == State::Ducking);
        hero.update();
        assert!(hero.state == State::Ducking);
        hero.update();
        assert!(hero.state == State::Ducking);
        hero.update();
        assert!(hero.state == State::Ducking);
        hero.handle_input(Input::ReleaseDown);
        assert!(hero.state == State::Standing);
        hero.update();
        assert!(hero.state == State::Standing);
        hero.handle_input(Input::PressB);
        assert!(hero.state == State::Jumping);
        hero.update();
        assert!(hero.state == State::Jumping);
        hero.handle_input(Input::PressDown);
        assert!(hero.state == State::Diving);
        hero.update();
        assert!(hero.state == State::Diving);
    }
}
