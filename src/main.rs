use std::fs::File;
//use std::io::Read;
use std::io::prelude::*;

use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::*;

fn main() -> std::io::Result<()> {

    let dialect = PostgreSqlDialect {}; // or AnsiDialect, or your own dialect ...

    let mut fit = std::env::args();
    fit.next();
    
    while let Some(fname) = fit.next() {
        // println!("file = {}", fname);
        let mut fh = File::open(fname)?;
        let mut sql = String::new();
        fh.read_to_string(&mut sql)?;
        
        for ddl in sql.split("\n-- ddl-end --\n") {
            match Parser::parse_sql(&dialect, ddl.to_string()) {
                Ok(ast) => {
                    if ast.len() < 1 {
                    }
                    else {
                        let ast0 = &ast[0];
                        if let Statement::CreateTable {name, columns, ..} = &ast0 {
                            let nv = &name.0;
                            println!("{}", nv[nv.len() - 1]);
                            for col in columns {
                                println!("{}\t{}", col.name, col.data_type);
                            }
                            println!("");
                        } else {
                            // println!("====\n{}\n====", ddl);
                            // println!("Unsupported AST: {:?}", ast)
                        }
                    }
                },
                Err(_) => (),
            }
        }
    }
    
    Ok(())
}
