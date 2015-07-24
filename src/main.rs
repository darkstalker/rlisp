extern crate rlisp;

use std::rc::Rc;
use std::cell::RefCell;
use rlisp::parser::Parser;
use rlisp::scope::GlobalScope;

fn main()
{
    let mut stdin = std::io::stdin();
    let env = Rc::new(RefCell::new(GlobalScope::new()));
    env.borrow_mut().load_stdlib();

    loop
    {
        let mut text = String::new();
        match stdin.read_line(&mut text) {
            Ok(0) => break,
            Ok(_) => {},
            Err(e) => panic!("Error reading input: {}", e),
        }

        match Parser::new(&text).parse() {
            Ok(vs) => for val in vs
            {
                match val.eval(env.clone()) {
                    Ok(v) => println!("Result: {}", v),
                    Err(e) => println!("Error: {}", e),
                }
            },
            Err(e) => println!("Error: {}", e),
        }
    }
}
