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
pub mod component;
