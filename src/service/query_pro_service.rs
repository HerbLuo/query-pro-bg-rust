use crate::model::query_structure::{QueryStructure, WhereClause, QueryStructureAction};
use serde_json::Value;
use crate::ec;
use crate::helper::resp::{HttpErrorData, JsonResult};
use crate::types::Uid;
use crate::model::permissions::Permissions;
use std::collections::HashMap;

fn build_select_fields(query_structure: &QueryStructure) -> Result<String, HttpErrorData> {
    let fields = &query_structure.fields;

    if fields.len() == 0 {
        return Ok("*".to_string());
    }

    // 这里注意，为了权限控制，应该过滤掉前端传来的不合法的字段, 比如子查询等

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

fn build_from_clause(
    table_permission_map: &HashMap<String, Vec<Permissions>>,
    query_structure: &QueryStructure
) -> Result<String, HttpErrorData> {
    let mut sql = String::new();
    sql.push_str(query_structure.from.main.as_str());

    let are_select = query_structure.action == QueryStructureAction::SELECT;
    let mut tables = vec![&query_structure.from.main];
    for joiner in &query_structure.from.joins {
        tables.push(&joiner.table);
    }
    for table in tables {
        if let Some(permissions) = table_permission_map.get(table) {
            for permission in permissions {
                let joiners_opt = &permission.joiners;
                if let Some(joiners) = joiners_opt {
                    if permission.uid_read.is_none() && are_select {
                        continue;
                    }
                    if permission.uid_write.is_none() && !are_select {
                        continue;
                    }

                    sql.push(' ');
                    sql.push_str(joiners);
                }
            }
        }
    }

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

fn build_where_clause(
    uid: &Uid,
    table_permission_map: &HashMap<String, Vec<Permissions>>,
    query_structure: &QueryStructure
) -> Result<String, HttpErrorData> {
    let mut where_clause_sql = String::from("WHERE ");

    let mut has_add_first_where_clause = false;

    fn add_table_permission_to_where_clause(
        sql: &mut String,
        table_permissions_map: &HashMap<String, Vec<Permissions>>,
        table_name: &String,
        uid_in_sql_opt: &Option<String>,
        has_add_first_where_clause: &mut bool
    ) -> Result<(), HttpErrorData> {
        let permissions_opt = table_permissions_map.get(table_name);

        if let Some(permissions) = permissions_opt {
            for permission in permissions {
                if let Some(uid_read) = &permission.uid_read {
                    if *has_add_first_where_clause {
                        sql.push_str(" AND ")
                    } else {
                        *has_add_first_where_clause = true
                    }
                    sql.push_str(uid_read);
                    sql.push_str(" = ");
                    if let Some(uid_in_sql) = uid_in_sql_opt {
                        sql.push_str(uid_in_sql);
                    } else {
                        return Err(fail!(ec::Unauthorized, format!("读取表 {} 需要登陆", table_name)));
                    }
                }
            }
        }

        return Ok(())
    }

    add_table_permission_to_where_clause(
        &mut where_clause_sql,
        table_permission_map,
        &query_structure.from.main,
        &uid.uid_sql_val_str,
        &mut has_add_first_where_clause
    )?;
    for joiner in &query_structure.from.joins {
        add_table_permission_to_where_clause(
            &mut where_clause_sql,
            table_permission_map,
            &joiner.table,
            &uid.uid_sql_val_str,
            &mut has_add_first_where_clause
        )?;
    }

    let where_clause_vec = &query_structure.where_clause;
    if where_clause_vec.len() == 0 {
        if where_clause_sql == "WHERE " {
            return Ok(String::new())
        }
        return Ok(where_clause_sql)
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

    let has_permission_control = has_add_first_where_clause;
    for (i, where_clause) in where_clause_vec.iter().enumerate() {
        if has_add_first_where_clause {
            if where_clause.operator == "or" {
                continue;
            }
            if i > 1 && &where_clause_vec[i - 1].operator == "or" {
                continue;
            }
            if i == 0 { // 防止注定的or语句破坏权限控制
                where_clause_sql.push_str(" AND (");
            } else {
                where_clause_sql.push_str(" AND ");
            }
        } else {
            has_add_first_where_clause = true;
        }
        parse_where_clause(&mut where_clause_sql, where_clause)?;
    }
    if has_permission_control {
        where_clause_sql.push(')');
    }

    println!("{}", where_clause_sql);
    return Ok(where_clause_sql)
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

fn query_structure_to_sql(
    uid: &Uid,
    permissions: &HashMap<String, Vec<Permissions>>,
    query_structure: &QueryStructure,
    _: bool
) -> Result<String, HttpErrorData> {
    println!("{:?}", query_structure);

    let action = &query_structure.action;
    let select_fields = build_select_fields(query_structure)?;
    let from_clause = build_from_clause(permissions, query_structure)?;
    let where_clause = build_where_clause(uid, permissions, query_structure)?;
    let order_by_clause = build_order_by_clause(query_structure)?;
    let limit_clause = build_limit_clause(query_structure);

    Ok(format!(
        "{:?} {}\nFROM {}\n{}\n{}\n{}",
        action, select_fields,
        from_clause,
        where_clause,
        order_by_clause,
        limit_clause
    ))
}

pub fn query(
    uid: &Uid,
    permissions: &HashMap<String, Vec<Permissions>>,
    query_structure: QueryStructure,
    by_remote: bool
) -> JsonResult<Vec<String>> {
    let sql = query_structure_to_sql(uid, permissions, &query_structure, by_remote)?;
    Ok(success!(vec![sql]))
}
