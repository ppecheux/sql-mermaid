use sqlparser::ast::{ColumnOption, ColumnOptionDef, Ident, Statement};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

fn ast(sql: &str) -> Vec<Statement> {
    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    log::info!("Update: {:?}", ast);
    return ast;
}

fn statement_mermaid(statement: Statement) -> String {
    let mut mermaid: String = String::new();
    let mut mermaid_constraints: String = String::new();
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
            mermaid.push_str(&format!("\t{} {{\n", name.to_string().replace('"', "")));
            for column in columns {
                mermaid.push_str(&format!(
                    "\t\t{} {}\n",
                    column
                        .data_type
                        .to_string()
                        .replace(' ', "_")
                        .replace('(', "")
                        .replace(')', "")
                        .replace(',', "_"),
                    column.name.to_string().replace('"', ""),
                ));
                for ColumnOptionDef {
                    name: option_name,
                    option,
                } in column.options
                {
                    match option {
                        ColumnOption::ForeignKey {
                            foreign_table,
                            referred_columns,
                            on_delete,
                            on_update,
                        } => mermaid_constraints
                            .push_str(&format!("\t{} ||--|{{ {} : \"\"\n", name, foreign_table)),
                        _ => {}
                    }
                }
            }
            mermaid.push_str("}");
            mermaid.push_str(mermaid_constraints.as_str());
            // ColumnDef { name: Ident { value: "CUSTOMER_ID", quote_style: None }, data_type: Int, collation: None, options: [ColumnOptionDef { name: None, option: ForeignKey { foreign_table: ObjectName([Ident { value: "CUSTOMERS", quote_style: None }]), referred_columns: [Ident { value: "ID", quote_style: None }], on_delete: None, on_update: None } }] }
            for constraint in constraints {
                match constraint {
                    sqlparser::ast::TableConstraint::ForeignKey {
                        name: fk_name,
                        columns,
                        foreign_table,
                        referred_columns,
                    } => {
                        for (column, referred_column) in columns.iter().zip(referred_columns.iter())
                        {
                            // let f = fk_name.take().to_owned();
                            let fk_display_name = match fk_name {
                                None => "",
                                Some(Ident { ref value, .. }) => value.as_str(), //&ident.value.as_str()
                            };
                            mermaid.push_str(&format!(
                                "\t{} ||--|{{ {} : \"{}\"\n",
                                name, foreign_table, fk_display_name
                            ))
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    mermaid
}

pub fn sql_s_mermaid(sql: &str) -> String {
    let mut mermaid: String = "erDiagram\n".to_string();
    for statement in ast(sql) {
        mermaid.push_str(statement_mermaid(statement).as_str());
    }
    mermaid
}
