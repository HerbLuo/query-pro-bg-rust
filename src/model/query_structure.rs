use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryStructure {
  pub action: QueryStructureAction,
  pub fields: Vec<Field>,
  pub from: QueryStructureFrom,
  #[serde(rename="where")]
  pub where_clause: Vec<WhereClause>,
  pub order_by: Vec<OrderByClause>,
  pub limit: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderByClause {
  pub field: Field,
  pub operator: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
  pub table: Option<String>,
  pub column: String,
  pub commands: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WhereClause {
  pub field: Option<Field>,
  pub operator: String,
  pub value: Option<Value>, // null arrayOr<string boolean integer long date> WhereClause[]
  pub commands: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum QueryStructureAction {
  SELECT,
  UPDATE,
  DELETE,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FromJoinerOn {
  pub left: Field,
  pub right: Field,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FromJoiner {
  #[serde(rename="type")]
  pub join_type: String,
  pub table: String,
  pub on: Vec<FromJoinerOn>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryStructureFrom {
  pub main: String,
  pub joins: Vec<FromJoiner>,
}
