use std::fmt;
use std::rc::Rc;
use builtins::BuiltinFn;
use lambda::Lambda;
use scope::RcScope;

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

#[derive(Debug, Clone, PartialEq)]
pub enum Value
{
    Nil,
    Bool(bool),
    Number(f64),
    Symbol(Rc<String>),
    String(Rc<String>),
    Builtin(Rc<BuiltinFn>),
    Lambda(Rc<Lambda>),
    List(List),
}

impl fmt::Display for Value
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(val) => write!(f, "{}", if val { "#t" } else { "#f" }),
            Value::Number(ref val) => write!(f, "{}", val),
            Value::Symbol(ref val) => write!(f, "{}", val),
            Value::String(ref val) => write!(f, "\"{}\"", val),
            Value::Builtin(ref val) => write!(f, "#<builtin:{}>", val.name),
            Value::Lambda(_) => write!(f, "#<lambda>"),
            Value::List(ref val) => write!(f, "{}", val),
        }
    }
}

pub trait Function
{
    fn call(&self, args: &List, env: RcScope, do_ev: bool) -> Result<Value, RuntimeError>;
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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
    InvalidComp(&'static str, &'static str),
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
            RuntimeError::InvalidComp(a, b) => write!(f, "Can't compare {} and {}", a, b),
        }
    }
}
