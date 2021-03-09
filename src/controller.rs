use crate::component::cors;
use crate::component::catchers;
use crate::model::permissions::Permissions;

pub mod query_pro_controller;

pub fn init(port: u16, permissions: &Vec<Permissions>) {
    let mut config = rocket::ignite().config().clone();
    config.set_port(port);
    info!("Rewriting config port");
    rocket::custom(config)
        .attach(cors::CORS())
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
