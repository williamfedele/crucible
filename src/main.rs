mod ast;
mod lexer;
mod parser;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let source = "
        fn main() -> void {
            let x: int = 3;
            let y: int = 4;
            let z: int = add(x,y);
            if (x < y) {
                z = z * 2;
            }
            while (x <= z) {
                x = add(x, 1);
            }
        }
        fn add(a: int, b: int) -> int {
            return a + b;
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
