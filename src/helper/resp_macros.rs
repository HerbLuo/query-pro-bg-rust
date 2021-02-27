macro_rules! success {
    ($data:expr) => {
        rocket_contrib::json::Json(crate::helper::resp::Data {
            success: 1,
            data: $data,
        })
    };
}

macro_rules! fail {
    ($code:expr) => { fail!($code, ()) };

    ($code:expr,$debug:expr) => {{
        let serial = uuid::Uuid::new_v4().to_simple().to_string();
        log::error!("{}:{}:{:?}", serial, $code.code, $debug);
        crate::helper::resp::WithStatus (
            $code.status,
            rocket_contrib::json::Json(crate::helper::resp::Data {
                success: 0,
                data: crate::helper::resp::HttpError {
                    code: $code.code,
                    serial,
                    tip: Some($code.reason),
                },
            }),
        )
    }};
}

macro_rules! rollback {
    ($code:expr) => { rollback!($code, ()) };

    ($code:expr,$display:expr) => { crate::helper::resp::Rollback {reason: fail!($code,$display)} };
}
