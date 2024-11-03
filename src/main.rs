mod ast;
// mod ir;
mod lexer;
mod parser;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let source = "
        let x: int = 3;
        let y: int = 4;
        let z: int = x + y;
        if (x < y) {
            z = z * 2;
        }
    ";
    let result = compile(source)?;
    println!("Compilation successful: {}", result);
    Ok(())
}

fn compile(source: &str) -> Result<String, Box<dyn Error>> {
    let tokens = lexer::lex(source)?;
    let ast = parser::parse(tokens)?;
    println!("{:?}", ast);
    Ok("ok".to_string())
}
