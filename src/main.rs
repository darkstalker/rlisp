extern crate rlisp;

use std::error::Error;
use rlisp::parser::Parser;

fn main()
{
    let mut stdin = std::io::stdin();
    loop
    {
        let mut text = String::new();
        match stdin.read_line(&mut text) {
            Ok(0) => break,
            Ok(_) => {},
            Err(e) => panic!("Error reading input: {}", e.description()),
        }

        match Parser::new(&text).parse() {
            Ok(vs) => for val in vs { println!("expr: {}", val) },
            Err(e) => println!("Error: {}", e),
        }
    }
}
