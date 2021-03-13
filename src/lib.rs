#![feature(proc_macro_hygiene, decl_macro)]
#![feature(once_cell)]

#[macro_use]
extern crate serde;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;

#[macro_use]
mod helper;

pub mod config;
mod model;
mod service;
mod controller;
mod component;

use helper::resp_error_code as ec;
use component::uid;
use std::thread;

pub use config::Config;

pub mod types {
    pub use crate::model::permissions::Permissions;
    pub use crate::config::ConfigBuilder;
    pub use crate::uid::Uid;
    pub use rocket::Request;
}

pub use uid::set_uid_getter;
pub use controller::query_pro_controller;

pub fn init_server_sync(config: Config) {
    if config.with_logger {
        helper::log::init();
    }

    let permissions = config.permissions;

    controller::init(config.port, permissions);
}

pub fn init_server(config: Config) {
    thread::spawn(move|| {
        init_server_sync(config);
    });
}
