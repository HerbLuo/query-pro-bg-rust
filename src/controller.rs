use crate::component::cors;
use crate::component::catchers;
use crate::model::permissions::Permissions;
use rocket::fairing::AdHoc;
use std::collections::HashMap;

pub mod query_pro_controller;

pub fn init(port: u16, permissions: Vec<Permissions>) {
    let mut config = rocket::ignite().config().clone();
    config.set_port(port);

    let table_permission_map: HashMap<String, Permissions> = permissions.into_iter()
        .map(|p| (p.table.to_lowercase(), p))
        .collect();

    info!("Rewriting config port");
    rocket::custom(config)
        .attach(cors::CORS())
        .attach(AdHoc::on_attach("with permissions", |rocket| {
            Ok(rocket.manage(table_permission_map))
        }))
        .register(catchers![
            catchers::unauthorized,
            catchers::forbidden,
            catchers::notfound,
        ])
        .mount(
            "/",
            routes![
                cors::options,
                query_pro_controller::query
            ],
        )
        .launch();
}
