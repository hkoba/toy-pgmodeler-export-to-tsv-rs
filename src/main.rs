use std::fs::File;
//use std::io::Read;
use std::io::prelude::*;

use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::*;

fn main() -> std::io::Result<()> {

    let dialect = PostgreSqlDialect {}; // or AnsiDialect, or your own dialect ...

    let mut fit = std::env::args();
    // $0 を読み捨て
    fit.next();
    
    // 以下、全ての引数を SQLファイルの名前と考えて
    while let Some(fname) = fit.next() {
        // println!("file = {}", fname);
        let mut fh = File::open(fname)?;
        let mut sql = String::new();
        fh.read_to_string(&mut sql)?;
        
        // 全体を一発で parse しようとすると CREATE ROLE がサポート外なので失敗する
        // たまたま pgModeler の　ddl-end 行がマーカーとして使えそうだったので、
        // それで全体を分割
        for ddl in sql.split("\n-- ddl-end --\n") {

            // parse に失敗した場合だけ印刷
            match Parser::parse_sql(&dialect, ddl.to_string()) {
                // 長さ 0 のものが返るケースも有った
                Ok(ast) if ast.len() >= 1 => {
                    let ast0 = &ast[0];
                    // 興味のない所は .. で省略
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
                },
                _ => ()
            }
        }
    }
    
    Ok(())
}
