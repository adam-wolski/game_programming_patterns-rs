extern crate rand;

mod command;
mod flyweight;
mod observer;
mod prototype;


pub fn main() {
    command::test();
    flyweight::test();
    observer::test();
    prototype::test();
}
