extern crate rand;

mod command;
mod flyweight;
mod observer;


pub fn main() {
    command::test();
    flyweight::test();
    observer::test();
}
