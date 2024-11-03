mod lexer;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let source = "
        fn add(a: int, b: int) -> int {
            return a + b;
        }
    ";
    let tokens = lexer::lex(source)?;
    println!("Compilation successful: {:?}", tokens);
    Ok(())
}
