use rocket::request::FromRequest;
use rocket::{Request, request};
use std::lazy::SyncOnceCell;
use rocket::http::Status;

pub struct Uid {
    pub uid_sql_val_str: String,
}

static UID_GETTER: SyncOnceCell<Box<dyn Sync + Send + Fn(&Request) -> Uid>> = SyncOnceCell::new();

pub fn set_uid_getter<F: 'static + Sync + Send + Fn(&Request) -> Uid>(uid_getter: F) {
    if let Err(_) = UID_GETTER.set(Box::new(uid_getter)) {
        panic!("set uid_getter failed.")
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Uid {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let uid_getter = match UID_GETTER.get() {
            Some(u) => u,
            None => return request::Outcome::Failure((Status::InternalServerError, ()))
        };
        request::Outcome::Success(uid_getter(request))
    }
}
