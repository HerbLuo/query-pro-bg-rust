#![feature(proc_macro_hygiene, decl_macro)]

mod types;
mod helper;
mod service;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate rocket;

mod controller;
mod component;

fn main() {
    helper::log::init();
    println!("Hello, world!");
    controller::init();
}
