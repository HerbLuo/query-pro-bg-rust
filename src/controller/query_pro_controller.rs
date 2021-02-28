use rocket_contrib::json::Json;
use crate::types::query_structure::QueryStructure;
use crate::service::query_pro_service;
use crate::helper::resp::JsonResult;

#[post("/query_pro", data = "<query_structure>")]
pub fn query(query_structure: Json<QueryStructure>) -> JsonResult<Vec<String>> {
    query_pro_service::query(query_structure.0, true)
}
