use crate::component::cors;

mod query_pro_controller;

pub fn init() {
    rocket::ignite()
        .attach(cors::CORS())
        // .register(catchers![
        //     catchers::unauthorized,
        //     catchers::forbidden,
        //     catchers::notfound,
        // ])
        .mount(
            "/",
            routes![
                cors::options,
                query_pro_controller::query
            ],
        )
        .launch();
}

