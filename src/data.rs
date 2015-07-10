use std::fmt;
use std::error::Error;

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

#[derive(Debug)]
pub enum Value
{
    Number(f64),
    Ident(String),
    String(String),
    List(Vec<Value>),
}

impl Value
{
    pub fn quote(val: Value) -> Value
    {
        Value::List(vec![Value::Ident("quote".to_string()), val])
    }
}

impl fmt::Display for Value
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self {
            Value::Number(ref val) => write!(f, "{}", val),
            Value::Ident(ref val) => write!(f, "{}", val),
            Value::String(ref val) => write!(f, "\"{}\"", val),
            Value::List(ref lst) => {
                let mut it = lst.iter();
                match it.next() {
                    Some(val) => write!(f, "({})", it.fold(format!("{}", val), |acc, val| acc + &format!(" {}", val))),
                    None => write!(f, "()"),
                }
            }
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
