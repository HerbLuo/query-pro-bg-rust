use query_pro_bg_rust::Config;
use crate::helpers::get_token::get_token;
use query_pro_bg_rust::types::Permissions;

mod helpers;

fn main() {
    let permissions = vec![
        Permissions {
            table: String::from("user_pri"),
            column: None,
            uid_read: Some(String::from("user_pri.uid")),
            uid_write: Some(String::from("user_pri.uid")),
            joiners: None
        },
        Permissions {
            table: String::from("user_pri_setting"),
            column: None,
            uid_read: Some(String::from("user_pri_setting.uid")),
            uid_write: Some(String::from("user_pri_setting.uid")),
            joiners: None
        },
    ];

    let config = Config::build()
        .with_logger(true)
        .port(8888)
        .permissions(permissions)
        .finalize();

    query_pro_bg_rust::set_uid_getter(get_token);
    query_pro_bg_rust::init_server_sync(config);
}
