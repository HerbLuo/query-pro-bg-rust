use rocket_contrib::json::Json;
use crate::model::query_structure::QueryStructure;
use crate::service::query_pro_service;
use crate::helper::resp::JsonResult;
use crate::component::uid::Uid;
use crate::model::permissions::Permissions;
use rocket::State;
use std::collections::HashMap;

#[post("/query-pro", data = "<query_structure>")]
pub fn query(
    uid: Uid,
    permissions: State<HashMap<String, Vec<Permissions>>>,
    query_structure: Json<QueryStructure>
) -> JsonResult<Vec<String>> {
    query_pro_service::query(&uid, &permissions, query_structure.0, true)
}
