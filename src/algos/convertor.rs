use std::collections::{HashMap, HashSet};

use sqlparser::ast::{ColumnOption, ColumnOptionDef, DataType, Ident, Statement};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

fn ast(sql: &str) -> Vec<Statement> {
    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    // log::info!("Update: {:?}", ast);
    return ast;
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum ColumnConstraint {
    NotNull,
    Unique,
    ForeignKey,
    PrimaryKey,
}

fn statement_mermaid(
    statement: Statement,
    table_column_type: &mut HashMap<String, HashMap<String, DataType>>,
    table_colum_constraints: &mut HashMap<String, HashMap<String, HashSet<ColumnConstraint>>>,
    column_foreign_key_column: &mut Vec<(TableColumn, TableColumn, String)>,
) -> String {
    let mut mermaid: String = String::new();

    match statement {
        Statement::CreateTable {
            or_replace,
            temporary,
            external,
            if_not_exists,
            name,
            columns,
            constraints,
            hive_distribution,
            hive_formats,
            table_properties,
            with_options,
            file_format,
            location,
            query,
            without_rowid,
            like,
        } => {
            let mut column_constraints: HashMap<String, HashSet<ColumnConstraint>> = HashMap::new();
            let mut column_type: HashMap<String, DataType> = HashMap::new();
            mermaid.push_str(&format!("\t{} {{\n", name.to_string().replace('"', "")));

            for column in columns {
                column_type.insert(column.name.to_string(), column.data_type);
                // mermaid.push_str(&format!(
                //     "\t\t{} {}\n",
                //     column
                //         .data_type
                //         .to_string()
                //         .replace(' ', "_")
                //         .replace('(', "")
                //         .replace(')', "")
                //         .replace(',', "_"),
                //     column.name.to_string().replace('"', ""),
                // ));
                for ColumnOptionDef {
                    name: option_name,
                    option,
                } in column.options
                {
                    match option {
                        ColumnOption::ForeignKey {
                            foreign_table,
                            mut referred_columns,
                            on_delete,
                            on_update,
                        } => {
                            let referred_column = match referred_columns.pop() {
                                Some(ref ident) => ident.to_string(),
                                None => "".to_string(),
                            }; // how to deal when referencing many cols ?
                            column_foreign_key_column.push((
                                (name.to_string(), column.name.to_string()),
                                (foreign_table.to_string(), referred_column),
                                "".to_string(),
                            ));
                            let constraint_set = column_constraints
                                .entry(column.name.to_string())
                                .or_insert(HashSet::new());
                            constraint_set.insert(ColumnConstraint::ForeignKey);
                        }
                        ColumnOption::NotNull => {
                            let constraint_set = column_constraints
                                .entry(column.name.to_string())
                                .or_insert(HashSet::new());
                            constraint_set.insert(ColumnConstraint::NotNull);
                        }
                        ColumnOption::Unique { is_primary } => {
                            let constraint_set = column_constraints
                                .entry(column.name.to_string())
                                .or_insert(HashSet::new());
                            constraint_set.insert(ColumnConstraint::Unique);
                            if is_primary {
                                constraint_set.insert(ColumnConstraint::NotNull);
                                constraint_set.insert(ColumnConstraint::PrimaryKey);
                            }
                        }
                        _ => {}
                    }
                }
            }
            mermaid.push_str("}");
            // ColumnDef { name: Ident { value: "CUSTOMER_ID", quote_style: None }, data_type: Int, collation: None, options: [ColumnOptionDef { name: None, option: ForeignKey { foreign_table: ObjectName([Ident { value: "CUSTOMERS", quote_style: None }]), referred_columns: [Ident { value: "ID", quote_style: None }], on_delete: None, on_update: None } }] }
            for constraint in constraints {
                match constraint {
                    sqlparser::ast::TableConstraint::ForeignKey {
                        name: fk_name,
                        columns,
                        foreign_table,
                        referred_columns,
                    } => {
                        let fk_display_name = match fk_name {
                            None => "".to_string(),
                            Some(Ident { ref value, .. }) => value.to_string(),
                        };
                        for (column, referred_column) in columns.iter().zip(referred_columns.iter())
                        {
                            column_foreign_key_column.push((
                                (name.to_string(), column.to_string()),
                                (foreign_table.to_string(), referred_column.to_string()),
                                fk_display_name.to_owned(),
                            ));
                            let constraint_set = column_constraints
                                .entry(column.to_string())
                                .or_insert(HashSet::new());
                            constraint_set.insert(ColumnConstraint::ForeignKey);
                        }
                    }
                    sqlparser::ast::TableConstraint::Unique {
                        name,
                        columns,
                        is_primary,
                    } => {
                        if !columns.is_empty() && columns.len() == 1 {
                            let cols = column_constraints
                                .entry(columns[0].to_string())
                                .or_insert(HashSet::new());
                            cols.insert(ColumnConstraint::Unique);
                            if is_primary {
                                cols.insert(ColumnConstraint::NotNull);
                                cols.insert(ColumnConstraint::PrimaryKey);
                            }
                        }
                    }
                    _ => {}
                }
            }
            table_colum_constraints.insert(name.to_string(), column_constraints);
            table_column_type.insert(name.to_string(), column_type);
        }
        _ => {}
    }
    mermaid
}

type TableColumn = (String, String);

fn zero_or_one_relation(
    table: &String,
    column: &String,
    table_colum_constraints: &HashMap<String, HashMap<String, HashSet<ColumnConstraint>>>,
) -> String {
    match table_colum_constraints.get(table) {
        None => "o".to_string(),
        Some(columns) => match columns.get(column) {
            None => "o".to_string(),
            Some(constraints) => {
                if constraints.contains(&ColumnConstraint::NotNull) {
                    "|".to_string()
                } else {
                    "o".to_string()
                }
            }
        },
    }
}

enum Side {
    Left,
    Right,
}

fn one_or_many_relation(
    side: Side,
    table: &String,
    column: &String,
    table_colum_constraints: &HashMap<String, HashMap<String, HashSet<ColumnConstraint>>>,
) -> String {
    let many = match side {
        Side::Left => "}",
        Side::Right => "{",
    };
    match table_colum_constraints.get(table) {
        None => many.to_string(),
        Some(columns) => match columns.get(column) {
            None => many.to_string(),
            Some(constraints) => {
                if constraints.contains(&ColumnConstraint::Unique) {
                    "|".to_string()
                } else {
                    many.to_string()
                }
            }
        },
    }
}

pub fn sql_s_mermaid(sql: &str) -> String {
    let mut table_column_constraints: HashMap<String, HashMap<String, HashSet<ColumnConstraint>>> =
        HashMap::new();
    let mut column_foreign_key_column: Vec<(TableColumn, TableColumn, String)> = Vec::new();
    let mut table_column_type: HashMap<String, HashMap<String, DataType>> = HashMap::new();

    let mut mermaid: String = "erDiagram\n".to_string();

    for statement in ast(sql) {
        statement_mermaid(
            statement,
            &mut table_column_type,
            &mut table_column_constraints,
            &mut column_foreign_key_column,
        );
    }
    log::debug!("Update: {:?}", table_column_constraints);

    // draw boxes
    for (table_name, table) in &table_column_type {
        mermaid.push_str(&format!(
            "\t{} {{\n",
            table_name.to_string().replace('"', "")
        ));
        let default_table_constraints = HashMap::new();
        let table_constraints = table_column_constraints.get(table_name).unwrap_or(&default_table_constraints);
        for (column_name, data_type) in table {
            let default_constraints = &HashSet::new();
            let constraints = &table_constraints.get(column_name).unwrap_or(&default_constraints);
            mermaid.push_str(&format!(
                "\t\t{} {} {} {}\n",
                data_type
                    .to_string()
                    .replace(' ', "_")
                    .replace('(', "")
                    .replace(')', "")
                    .replace(',', "_"),
                column_name.to_string().replace('"', ""),
                if constraints.contains(&ColumnConstraint::PrimaryKey) {
                    "PK"
                } else {
                    ""
                },
                if constraints.contains(&ColumnConstraint::ForeignKey) {
                    "\"FK\""
                } else {
                    ""
                }
            ));
        }
        mermaid.push_str("}");
    }

    // draw links
    for ((l_table, l_column), (r_table, r_column), foreign_key) in column_foreign_key_column {
        mermaid.push_str(&format!(
            "\t{} }}o--{}{} {} : \"{}\"\n",
            l_table,
            // one_or_many_relation(Side::Left, &l_table, &l_column, &table_column_constraints,),
            // zero_or_one_relation(&l_table, &l_column, &table_column_constraints,),
            zero_or_one_relation(&r_table, &r_column, &table_column_constraints,),
            one_or_many_relation(Side::Right, &r_table, &r_column, &table_column_constraints,),
            r_table,
            foreign_key
        ))
    }
    mermaid
}
