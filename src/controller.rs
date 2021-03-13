use crate::component::cors;
use crate::component::catchers;
use crate::model::permissions::Permissions;
use rocket::fairing::AdHoc;
use std::collections::HashMap;

pub mod query_pro_controller;

pub fn init(port: u16, permissions: Vec<Permissions>) {
    let mut config = rocket::ignite().config().clone();
    config.set_port(port);

    let mut table_permission_map: HashMap<String, Vec<Permissions>> = HashMap::new();
    for permission in permissions {
        let key = permission.table.to_lowercase();
        let mut value_opt = table_permission_map.get(&key).unwrap_or(&vec![]).to_vec();
        value_opt.push(permission);
        table_permission_map.insert(key, value_opt);
    }

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
