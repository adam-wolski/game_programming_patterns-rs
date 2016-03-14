//! Finite State Machine
//! First example from http://gameprogrammingpatterns.com/state.html using standard switch or match

use state::{Input, Graphic};
const JUMP_VELOCITY: i32 = 10;
const MAX_CHARGE: u32 = 3;

#[derive(Debug)]
enum State {
    Jumping,
    Standing,
    Ducking,
    Diving,
}

#[derive(Debug)]
struct Heroine {
    state: State,
    graphic: Graphic,
    y_velocity: i32,
    charge_time: u32
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
        match self.state {
            State::Ducking => {
                self.charge_time += 1;
                if self.charge_time > MAX_CHARGE {
                    self.super_bomb();
                }
            }
            // ... other
            _ => {}
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
                match input {
                    Input::PressDown => {
                        self.state = State::Diving;
                        println!("Changing state to Diving");
                        self.set_graphics(Graphic::Dive);
                    }
                    _ => {}
                }
            }
            State::Ducking => {
                match input {
                    Input::ReleaseDown => {
                        self.state = State::Standing;
                        println!("Changing state to Standing");
                        self.set_graphics(Graphic::Stand);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

pub fn test() {
    println!("\n---------------------------");
    println!("Finite state machine test.\n");
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
