#![feature(proc_macro_hygiene, decl_macro)]
#![feature(once_cell)]

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
use crate::component::uid::{set_uid_getter, Uid};

fn main() {
    helper::log::init();

    set_uid_getter(|request| {
        Uid{ uid_sql_val_str: "test".to_string() }
    });

    println!("Hello, world!");
    controller::init();
}
