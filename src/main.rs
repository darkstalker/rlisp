extern crate rlisp;

use std::error::Error;
use rlisp::parser::Parser;
use rlisp::scope::GlobalScope;

fn main()
{
    let mut stdin = std::io::stdin();
    let mut env = GlobalScope::new();
    env.load_stdlib();

    loop
    {
        let mut text = String::new();
        match stdin.read_line(&mut text) {
            Ok(0) => break,
            Ok(_) => {},
            Err(e) => panic!("Error reading input: {}", e.description()),
        }

        match Parser::new(&text).parse() {
            Ok(vs) => for val in vs
            {
                match val.eval(&mut env) {
                    Ok(v) => println!("Result: {}", v),
                    Err(e) => println!("Error: {}", e),
                }
            },
            Err(e) => println!("Error: {}", e),
        }
    }
}
