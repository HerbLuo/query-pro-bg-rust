#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate rocket;

#[macro_use]
mod helper;

mod config;
mod types;
mod service;
mod controller;
mod component;

use helper::resp_error_code as ec;

fn main() {
    helper::log::init();
    println!("Hello, world!");
    controller::init();
}
