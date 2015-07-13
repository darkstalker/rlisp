use std::fmt;
use std::error::Error;
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
    Ident(String),
    String(String),
    Builtin(BuiltinFn, bool),
    List(Option<Rc<Cons>>),
}

#[derive(Clone)]
pub struct BuiltinFn(pub Rc<Fn(&Option<Rc<Cons>>) -> Result<Value, RuntimeError>>);

impl fmt::Debug for BuiltinFn
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "<BuiltinFn>")
    }
}

impl fmt::Display for Value
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self {
            Value::Nil => write!(f, "#<Nil>"),
            Value::Number(ref val) => write!(f, "{}", val),
            Value::Ident(ref val) => write!(f, "{}", val),
            Value::String(ref val) => write!(f, "\"{}\"", val),
            Value::Builtin(_, _) => write!(f, "#<BuiltinFn>"),
            Value::List(ref opt) => match *opt {
                Some(ref val) => write!(f, "({})", val),
                None => write!(f, "()"),
            }
        }
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

impl Error for ParseError
{
    fn description(&self) -> &str
    {
        match *self {
            ParseError::UnclosedString => "Unclosed string",
            ParseError::InvalidNumber => "Invalid number literal",
            ParseError::UnclosedList => "Unclosed list",
            ParseError::UnexpectedRparen => "Unexpected ')'",
            ParseError::NoQuoteArg => "Missing quote argument",
            ParseError::EndOfStream => "End of stream",
        }
    }
}

impl fmt::Display for ParseError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.description())
    }
}

#[derive(Debug)]
pub enum RuntimeError
{
    UnkIdent(String),
    InvalidCall,
    MissingArgs,
    InvalidArg,
    Unimplemented,
}
