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
    Ident(Rc<String>),
    String(Rc<String>),
    Builtin(Rc<BuiltinFn>),
    List(Option<Rc<Cons>>),
}

impl fmt::Display for Value
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self {
            Value::Nil => write!(f, "nil"),
            Value::Number(ref val) => write!(f, "{}", val),
            Value::Ident(ref val) => write!(f, "{}", val),
            Value::String(ref val) => write!(f, "\"{}\"", val),
            Value::Builtin(ref val) => write!(f, "#builtin:{}", val.name),
            Value::List(ref opt) => match *opt {
                Some(ref val) => write!(f, "({})", val),
                None => write!(f, "()"),
            }
        }
    }
}

pub struct BuiltinFn
{
    pub name: &'static str,
    pub do_eval: bool,
    pub call: Box<Fn(&Option<Rc<Cons>>) -> Result<Value, RuntimeError>>,
}

impl BuiltinFn
{
    pub fn new<F>(n: &'static str, de: bool, f: F) -> Rc<BuiltinFn>
        where F: Fn(&Option<Rc<Cons>>) -> Result<Value, RuntimeError> + 'static
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
pub struct Cons
{
    pub car: Value,
    pub cdr: Option<Rc<Cons>>,
}

impl Cons
{
    pub fn new(car: Value, cdr: Option<Rc<Cons>>) -> Option<Rc<Cons>>
    {
        Some(Rc::new(Cons{ car: car, cdr: cdr }))
    }

    pub fn from_vec(mut vec: Vec<Value>) -> Option<Rc<Cons>>
    {
        let mut cdr = None;
        while let Some(car) = vec.pop()
        {
            cdr = Cons::new(car, cdr);
        }
        cdr
    }
}

impl fmt::Display for Cons
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self {
            Cons{ ref car, cdr: None } => write!(f, "{}", car),
            Cons{ ref car, cdr: Some(ref next) } => write!(f, "{} {}", car, next),
        }
    }
}

pub trait Scope<'a>
{
    fn get(&self, key: &str) -> Option<Value>;
    fn set(&mut self, key: &'a str, val: Value);
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
    UnkIdent(Rc<String>),
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
            RuntimeError::UnkIdent(ref s) => write!(f, "Unknown identifier: {}", s),
            RuntimeError::InvalidCall(t) => write!(f, "Invalid call on a {} value", t),
            RuntimeError::InvalidArgNum(n) => write!(f, "Incorrect number or arguments (Expected {})", n),
            RuntimeError::InvalidArgType(a, b) => write!(f, "Invalid argument: expected {}, but found {}", a, b),
            RuntimeError::Unimplemented => write!(f, "Unimplemented"),
        }
    }
}
