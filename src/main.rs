extern crate rand;

mod command;
mod flyweight;
mod observer;
mod prototype;
mod state;


pub fn main() {
    command::test();
    flyweight::test();
    observer::test();
    prototype::test();
    state::finite_state_machine::test();
}
