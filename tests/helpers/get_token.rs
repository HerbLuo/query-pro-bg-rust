use query_pro_bg_rust::types::{ Uid, Request };

pub fn get_token(_: &Request) -> Result<Uid, String> {
    Ok(Uid {uid_sql_val_str: Some("'6749028329662868001'".to_string())})
}
