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
            Value::List(ref lst) => match *lst {
                List::Node(ref val) => write!(f, "({})", val),
                List::End => write!(f, "()"),
            }
        }
    }
}

pub struct BuiltinFn
{
    pub name: &'static str,
    pub do_eval: bool,
    pub call: Box<Fn(&List, &mut Scope) -> Result<Value, RuntimeError>>,
}

impl BuiltinFn
{
    pub fn new<F>(n: &'static str, de: bool, f: F) -> Rc<BuiltinFn>
        where F: Fn(&List, &mut Scope) -> Result<Value, RuntimeError> + 'static
    {
        Rc::new(BuiltinFn{ name: n, do_eval: de, call: Box::new(f) })
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

#[derive(Debug, Clone)]
pub struct Cons
{
    pub car: Value,
    pub cdr: List,
}

impl List
{
    pub fn cons(car: Value, cdr: List) -> List
    {
        List::Node(Rc::new(Cons{ car: car, cdr: cdr }))
    }

    pub fn from_vec(mut vec: Vec<Value>) -> List
    {
        let mut cdr = List::End;
        while let Some(car) = vec.pop()
        {
            cdr = List::cons(car, cdr);
        }
        cdr
    }
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
    InvalidArgNum(u32),
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
            RuntimeError::InvalidArgNum(n) => write!(f, "Incorrect number or arguments (Expected {})", n),
            RuntimeError::InvalidArgType(a, b) => write!(f, "Invalid argument: expected {}, but found {}", a, b),
            RuntimeError::Unimplemented => write!(f, "Unimplemented"),
        }
    }
}
