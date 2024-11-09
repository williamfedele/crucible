mod ast;
mod ir;
mod lexer;
mod parser;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let source = "
        let x: int = 3;
        let unused: int = 0;
        let y: int = x + 1;
        let z: int = x * y / 2;
        z = z + 1;
    ";
    let result = compile(source)?;
    println!("Compilation successful: {}", result);
    Ok(())
}

fn compile(source: &str) -> Result<String, Box<dyn Error>> {
    let tokens = lexer::lex(source)?;
    let ast = parser::parse(tokens)?;
    let mut ir = ir::lower(ast);
    ir::optimize(&mut ir);
    Ok("ok".to_string())
}
