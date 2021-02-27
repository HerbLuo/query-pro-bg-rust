use crate::types::query_structure::{QueryStructure, WhereClause};
use serde_json::Value;
use rocket::response::Responder;
use crate::helper::resp::{HttpErrorData, JsonResult};

fn build_select_fields(query_structure: &QueryStructure, by_remote: bool) -> Result<String, HttpErrorData> {
    let fields = &query_structure.fields;

    if fields.len() == 0 {
        return Ok("*".to_string());
    }

    let mut sql = String::new();
    let last_field_index = fields.len() - 1;
    for (i, field) in fields.iter().enumerate() {
        if let Some(table) = &field.table {
            sql.push_str(table);
            sql.push('.');
        }
        sql.push_str(&field.column);
        if i != last_field_index {
            sql.push_str(", ");
        }
    }
    return Ok(sql)
}

fn build_from_clause(query_structure: &QueryStructure, by_remote: bool) -> String {
    return format!("{}", query_structure.from.main);
}

fn build_where_clause(query_structure: &QueryStructure, by_remote: bool) -> String {
    let where_clause = &query_structure.where_clause;
    if where_clause.len() == 0 {
        return String::new()
    }

    let parse_where_clause = |mut sql: &mut String, where_clause: &WhereClause| {
        if let Some(field) = &where_clause.field {
            if let Some(table) = &field.table {
                sql.push_str(table.as_str());
                sql.push('.');
            }
            sql.push_str(field.column.as_str());
        }
        let operator = &where_clause.operator;
        sql.push(' ');
        sql.push_str(operator.as_str());
        sql.push(' ');
        let handle_no_arr_value = |sql: &mut String, value: &Value| {
            if let Some(val_str) = value.as_str() {
                sql.push('\'');
                sql.push_str(val_str);
                sql.push('\'');
            }
            if let Some(val_num) = value.as_f64() {
                sql.push_str(val_num.to_string().as_str());
            }
            if let Some(val_bool) = value.as_bool() {
                sql.push_str(&val_bool.to_string().as_str());
            }
            if let Some(_) = value.as_null() {
                sql.push_str("null");
            }
        };
        if let Some(value) = &where_clause.value {
            if let Some(value_arr) = value.as_array() {
                sql.push('(');
                for value2 in value_arr {
                    handle_no_arr_value(&mut sql, value2);
                }
                sql.push(')');
            } else {
                handle_no_arr_value(&mut sql, value);
            }
        }
    };

    let mut where_clause_sql = String::new();
    let last_where_clause_index = where_clause.len() - 1;
    for (i, where_clause) in where_clause.iter().enumerate() {
        parse_where_clause(&mut where_clause_sql, where_clause);
        if i != last_where_clause_index {
            where_clause_sql.push_str(" and ");
        }
    }

    return format!("WHERE {}", where_clause_sql)
}

fn build_order_by_clause(query_structure: &QueryStructure, by_remote: bool) -> String {
    String::new()
}

fn build_limit_clause(query_structure: &QueryStructure, by_remote: bool) -> String {
    String::new()
}

fn query_structure_to_sql(query_structure: &QueryStructure, by_remote: bool) -> Result<String, HttpErrorData> {
    let action = &query_structure.action;
    let select_fields = build_select_fields(&query_structure, by_remote)?;
    let from_clause = build_from_clause(&query_structure, by_remote);
    let where_clause = build_where_clause(&query_structure, by_remote);
    let order_by_clause = build_order_by_clause(&query_structure, by_remote);
    let limit_clause = build_limit_clause(&query_structure, by_remote);

    Ok(format!(
        "{:?} {} FROM {} {} {} {}",
        action, select_fields, from_clause, where_clause, order_by_clause, limit_clause
    ))
}

pub fn query(query_structure: QueryStructure, by_remote: bool) -> JsonResult<String> {
    let sql = query_structure_to_sql(&query_structure, by_remote)?;
    println!("{}", sql);
    Ok(success!(sql))
}
