use crate::types::query_structure::{QueryStructure, WhereClause};
use serde_json::Value;

fn build_select_fields(query_structure: &QueryStructure, by_remote: bool) -> String {
    String::new()
}

fn build_from_clause(query_structure: &QueryStructure, by_remote: bool) -> String {
    return format!("{}", query_structure.from.main);
}

fn build_where_clause(query_structure: &QueryStructure, by_remote: bool) -> String {
    if query_structure.where_clause.len() == 0 {
        return String::new()
    }

    let parse_where_clause = |mut sql: &mut String, where_clause: &WhereClause| {
        if let Some(field) = &where_clause.field {
            if let Some(table) = &field.table {
                sql.push_str(table.as_str());
                sql.push_str(".");
            }
            sql.push_str(field.column.as_str());
        }
        let operator = &where_clause.operator;
        // if operator
        sql.push_str(operator.as_str());
        let handle_no_arr_value = |sql: &mut String, value: &Value| {
            if let Some(val_str) = value.as_str() {
                sql.push_str("'");
                sql.push_str(val_str);
                sql.push_str("'");
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
                sql.push_str("(");
                for value2 in value_arr {
                    handle_no_arr_value(&mut sql, value2);
                }
                sql.push_str(")");
            } else {
                handle_no_arr_value(&mut sql, value);
            }
        }
    };

    let mut where_clause_sql = String::new();
    for where_clause in &query_structure.where_clause {
        parse_where_clause(&mut where_clause_sql, where_clause);
        where_clause_sql.push_str(" and ");
    }

    return format!("WHERE {}", where_clause_sql)
}

fn build_order_by_clause(query_structure: &QueryStructure, by_remote: bool) -> String {
    String::new()
}

fn build_limit_clause(query_structure: &QueryStructure, by_remote: bool) -> String {
    String::new()
}

fn query_structure_to_sql(query_structure: &QueryStructure, by_remote: bool) -> String {
    let action = &query_structure.action;
    let select_fields = build_select_fields(&query_structure, by_remote);
    let from_clause = build_from_clause(&query_structure, by_remote);
    let where_clause = build_where_clause(&query_structure, by_remote);
    let order_by_clause = build_order_by_clause(&query_structure, by_remote);
    let limit_clause = build_limit_clause(&query_structure, by_remote);

    return format!(
        "{:?} {} FROM {} {} {} {}",
        action, select_fields, from_clause, where_clause, order_by_clause, limit_clause
    );
}

pub fn query(query_structure: QueryStructure, by_remote: bool) -> String {
    let sql = query_structure_to_sql(&query_structure, by_remote);
    println!("{}", sql);
    sql
}
