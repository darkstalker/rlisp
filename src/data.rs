use std::fmt;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Token
{
    Lparen,
    Rparen,
    Quote,
    Number(f64),
    Ident(String),
    String(String),
    Error(ParseError),
    End,    // end of string
}

#[derive(Debug, Clone)]
pub enum Value
{
    Nil,
    Number(f64),
    Symbol(Rc<String>),
    String(Rc<String>),
    Builtin(Rc<BuiltinFn>),
    List(List),
}

impl fmt::Display for Value
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self {
            Value::Nil => write!(f, "nil"),
            Value::Number(ref val) => write!(f, "{}", val),
            Value::Symbol(ref val) => write!(f, "{}", val),
            Value::String(ref val) => write!(f, "\"{}\"", val),
            Value::Builtin(ref val) => write!(f, "#builtin:{}", val.name),
            Value::List(ref val) => write!(f, "{}", val),
        }
    }
}

pub struct BuiltinFn
{
    pub name: &'static str,
    pub do_eval: bool,
    pub func: Box<Fn(&List, &mut Scope) -> Result<Value, RuntimeError>>,
}

impl BuiltinFn
{
    pub fn call(&self, args: &List, env: &mut Scope) -> Result<Value, RuntimeError>
    {
        if self.do_eval
        {
            (self.func)(&try!(args.eval(env)), env)
        }
        else
        {
            (self.func)(args, env)
        }
    }
}

impl fmt::Debug for BuiltinFn
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "BuiltinFn({})", self.name)
    }
}

#[derive(Debug, Clone)]
pub enum List
{
    Node(Rc<Cons>),
    End,
}

impl fmt::Display for List
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self {
            List::Node(ref val) => write!(f, "({})", val),
            List::End => write!(f, "()"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cons
{
    pub car: Value,
    pub cdr: List,
}

impl fmt::Display for Cons
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self {
            Cons{ ref car, cdr: List::Node(ref next) } => write!(f, "{} {}", car, next),
            Cons{ ref car, cdr: List::End } => write!(f, "{}", car),
        }
    }
}

pub trait Scope
{
    fn get(&self, key: &str) -> Option<Value>;
    fn set(&mut self, key: &str, val: Value);
}

#[derive(Debug, PartialEq)]
pub enum ParseError
{
    UnclosedString,
    InvalidNumber,
    UnclosedList,
    UnexpectedRparen,
    NoQuoteArg,
    EndOfStream,
}

impl fmt::Display for ParseError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self {
            ParseError::UnclosedString => write!(f, "Unclosed string"),
            ParseError::InvalidNumber => write!(f, "Invalid number literal"),
            ParseError::UnclosedList => write!(f, "Unclosed list"),
            ParseError::UnexpectedRparen => write!(f, "Unexpected ')'"),
            ParseError::NoQuoteArg => write!(f, "Missing quote argument"),
            ParseError::EndOfStream => write!(f, "End of stream"),
        }
    }
}

#[derive(Debug)]
pub enum RuntimeError
{
    UnkSymbol(Rc<String>),
    InvalidCall(&'static str),
    InvalidArgNum(u32, u32),
    InvalidArgType(&'static str, &'static str),
    Unimplemented,
}

impl fmt::Display for RuntimeError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self {
            RuntimeError::UnkSymbol(ref s) => write!(f, "Unbound variable: {}", s),
            RuntimeError::InvalidCall(t) => write!(f, "Invalid call on a {} value", t),
            RuntimeError::InvalidArgNum(n, a) => write!(f, "Expected {} arguments, but got {}", n, a),
            RuntimeError::InvalidArgType(a, b) => write!(f, "Invalid argument: expected {}, but found {}", a, b),
            RuntimeError::Unimplemented => write!(f, "Unimplemented"),
        }
    }
}
