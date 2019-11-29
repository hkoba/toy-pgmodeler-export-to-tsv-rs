use std::fs::File;
//use std::io::Read;
use std::io::prelude::*;

use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

fn main() -> std::io::Result<()> {

    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...
    
    let mut fit = std::env::args();
    fit.next();
    
    while let Some(fname) = fit.next() {
        println!("file = {}", fname);
        let mut fh = File::open(fname)?;
        let mut sql = String::new();
        fh.read_to_string(&mut sql)?;
        
        for ddl in sql.split("\n-- ddl-end --\n") {
            match Parser::parse_sql(&dialect, ddl.to_string()) {
                Ok(ast) => {
                    println!("====\n{}\n====", ddl);
                    println!("");
                    println!("AST: {:?}", ast)
                },
                Err(_) => (),
            }
        }
    }
    
    Ok(())
}
