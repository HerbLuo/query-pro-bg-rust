use crate::model::query_structure::{QueryStructure, WhereClause};
use serde_json::Value;
use crate::ec;
use crate::helper::resp::{HttpErrorData, JsonResult};
use crate::types::Uid;

fn build_select_fields(query_structure: &QueryStructure) -> Result<String, HttpErrorData> {
    let fields = &query_structure.fields;

    if fields.len() == 0 {
        return Ok("*".to_string());
    }

    // 这里注意，为了权限控制，应该过滤掉前端传来的不合法的字段

    let mut sql = String::new();
    let last_field_index = fields.len() - 1;
    for (i, field) in fields.iter().enumerate() {
        let upper_case = field.commands.as_ref().map(|c| c == "UPPER_CASE").unwrap_or(false);
        if upper_case {
            sql.push_str("UPPER(");
        }
        // here
        if let Some(table) = &field.table {
            sql.push_str(table);
            sql.push('.');
        }
        sql.push_str(&field.column);
        if upper_case {
            sql.push(')');
        }
        if i != last_field_index {
            sql.push_str(", ");
        }
    }
    return Ok(sql)
}

fn build_from_clause(query_structure: &QueryStructure) -> Result<String, HttpErrorData> {
    let mut sql = String::new();
    sql.push_str(query_structure.from.main.as_str());
    for joiner in &query_structure.from.joins {
        println!("joiner {:?}", joiner);
        let join_type = match joiner.join_type.as_str() {
            "LEFT_JOIN" => " LEFT JOIN",
            others => return Err(fail!(ec::NotImplemented, format!("not implemented({})", others)))
        };
        sql.push_str(join_type);
        sql.push(' ');
        sql.push_str(&joiner.table);
        sql.push_str(" ON ");
        let join_on_vec = &joiner.on;
        let last_join_on_index = join_on_vec.len() - 1;
        for (i, o) in join_on_vec.iter().enumerate() {
            let left = &o.left;
            if let Some(table) = &left.table {
                sql.push_str(table);
                sql.push('.');
            }
            sql.push_str(&left.column);
            sql.push_str(" = ");
            let right = &o.right;
            if let Some(table) = &right.table {
                sql.push_str(table);
                sql.push('.');
            }
            sql.push_str(&right.column);
            if i != last_join_on_index {
                sql.push_str(" AND ")
            }
        }

    }
    return Ok(sql);
}

fn build_where_clause(uid: &Uid, query_structure: &QueryStructure) -> Result<String, HttpErrorData> {
    let where_clause_vec = &query_structure.where_clause;
    if where_clause_vec.len() == 0 {
        return Ok(String::new())
    }

    fn parse_where_clause(mut sql: &mut String, where_clause: &WhereClause) -> Result<(), HttpErrorData> {
        let upper_case = where_clause.commands.as_ref()
            .map(|c| c == "UPPER_CASE")
            .unwrap_or(false);
        if let Some(field) = &where_clause.field {
            if upper_case {
                sql.push_str("UPPER(")
            }
            if let Some(table) = &field.table {
                sql.push_str(table.as_str());
                sql.push('.');
            }
            sql.push_str(field.column.as_str());
            if upper_case {
                sql.push(')')
            }
        }
        let operator = &where_clause.operator;
        sql.push(' ');
        sql.push_str(operator.as_str());
        sql.push(' ');
        let handle_no_arr_value = |sql: &mut String, value: &Value| -> Result<(), HttpErrorData> {
            if upper_case {
                sql.push_str("UPPER(");
            }
            let do_final_and_return = |sql: &mut String| {
                if upper_case {
                    sql.push(')');
                }
                return Ok(())
            };
            if let Some(val_str) = value.as_str() {
                sql.push('\'');
                sql.push_str(val_str);
                sql.push('\'');
                return do_final_and_return(sql);
            }
            if let Some(val_num) = value.as_f64() {
                sql.push_str(val_num.to_string().as_str());
                return do_final_and_return(sql);
            }
            if let Some(val_bool) = value.as_bool() {
                sql.push_str(&val_bool.to_string().as_str());
                return do_final_and_return(sql);
            }
            if let Some(_) = value.as_null() {
                sql.push_str("null");
                return do_final_and_return(sql);
            }
            let where_clause: WhereClause = serde_json::from_value(value.clone()) // todo the clone
                .map_err(|e| fail!(ec::NotImplemented, format!("not implemented({:?}) {:?}", value, e)))?;
            parse_where_clause(sql, &where_clause)?;
            Ok(())
        };
        if let Some(value) = &where_clause.value {
            if let Some(value_arr) = value.as_array() {
                sql.push('(');
                let connector = match operator.as_str() {
                    "in" => ", ",
                    "not in" => ", ",
                    "between" => " AND ",
                    "not between" => " AND ",
                    "or" => " AND ",
                    _ => return Err(fail!(ec::NotImplemented, format!("not implemented({})", operator)))
                };
                let last_arr_index = value_arr.len() - 1;
                for (i, value2) in value_arr.iter().enumerate() {
                    handle_no_arr_value(&mut sql, value2)?;
                    if i != last_arr_index {
                        sql.push_str(connector);
                    }
                }
                sql.push(')');
            } else {
                handle_no_arr_value(&mut sql, value)?;
            }
        }
        Ok(())
    };

    let mut where_clause_sql = String::new();
    let last_where_clause_index = where_clause_vec.len() - 1;
    for (i, where_clause) in where_clause_vec.iter().enumerate() {
        parse_where_clause(&mut where_clause_sql, where_clause)?;
        if i != last_where_clause_index {
            if where_clause.operator == "or" {
                continue;
            }

            if &where_clause_vec[i + 1].operator == "or" {
                continue;
            }

            where_clause_sql.push_str(" AND ");
        }
    }

    return Ok(format!("WHERE {}", where_clause_sql))
}

fn build_order_by_clause(query_structure: &QueryStructure) -> Result<String, HttpErrorData> {
    let order_by_vec = &query_structure.order_by;
    if order_by_vec.len() == 0 {
        return Ok(String::new())
    }

    let mut order_by_sql = String::from("ORDER BY ");
    let last_index = order_by_vec.len() - 1;
    for (i, order_by) in order_by_vec.iter().enumerate() {
        if let Some(table) = &order_by.field.table {
            order_by_sql.push_str(table.as_str());
            order_by_sql.push('.');
        }
        order_by_sql.push_str(order_by.field.column.as_str());
        order_by_sql.push(' ');
        let operator = match order_by.operator.as_str() {
            "desc" => "DESC",
            "asc" => "ASC",
            "" => "ASC",
            _ => return Err(fail!(ec::WrongParam, format!("un-support operator {}", order_by.operator)))
        };
        order_by_sql.push_str(operator);
        if i != last_index {
           order_by_sql.push_str(", ");
        }
    }

    Ok(order_by_sql)
}

fn build_limit_clause(query_structure: &QueryStructure) -> String {
    if let Some(limit) = query_structure.limit {
        if limit != -1 {
            return format!("LIMIT {}", limit)
        }
    }
    String::new()
}

fn query_structure_to_sql(uid: &Uid, query_structure: &QueryStructure, _: bool) -> Result<String, HttpErrorData> {
    println!("{:?}", query_structure);

    let action = &query_structure.action;
    let select_fields = build_select_fields(&query_structure)?;
    let from_clause = build_from_clause(&query_structure)?;
    let where_clause = build_where_clause(uid, &query_structure)?;
    let order_by_clause = build_order_by_clause(&query_structure)?;
    let limit_clause = build_limit_clause(&query_structure);

    Ok(format!(
        "{:?} {}\nFROM {}\n{}\n{}\n{}",
        action, select_fields,
        from_clause,
        where_clause,
        order_by_clause,
        limit_clause
    ))
}

pub fn query(uid: &Uid, query_structure: QueryStructure, by_remote: bool) -> JsonResult<Vec<String>> {
    let sql = query_structure_to_sql(uid, &query_structure, by_remote)?;
    Ok(success!(vec![sql]))
}
