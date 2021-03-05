use rocket::request::FromRequest;
use rocket::{Request, request};
use std::sync::Mutex;
use std::lazy::SyncLazy;

pub struct Uid {
    pub uid_sql_val_str: String,
}

pub trait UidGetter {
    fn uid_getter(&self, req: &Request) -> Uid;
}

pub struct DefaultUidGetter {

}

impl UidGetter for DefaultUidGetter {
    fn uid_getter(&self, req: &Request) -> Uid {
        Uid { uid_sql_val_str: "err".to_string() }
    }
}

static UID_GETTER: SyncLazy<Mutex<Box<dyn Fn(&Request) -> Uid>>> =
    SyncLazy::new(|| Mutex::new( Box::new(|_| Uid{ uid_sql_val_str: "err".to_string() })));

pub fn set_uid_getter<F: Fn(&Request) -> Uid>(uid_getter: F) {
  *UID_GETTER.lock().unwrap() = Box::new(|req| uid_getter(req));
}

impl<'a, 'r> FromRequest<'a, 'r> for Uid {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        request::Outcome::Success(UID_GETTER.uid_getter(request))
    }
}
