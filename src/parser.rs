use std::rc::Rc;
use std::mem;
use data::{Token, Value, List, ParseError};
use lexer::Tokenizer;

pub struct Parser<'a>
{
    cur_tok: Token,
    input: Tokenizer<'a>,
}

impl<'a> Parser<'a>
{
    pub fn new(text: &str) -> Parser
    {
        let mut tokens = Tokenizer::new(text);
        Parser{ cur_tok: tokens.next_token(), input: tokens }
    }

    // consumes the current token and pulls a new one
    fn next_token(&mut self) -> Token
    {
        mem::replace(&mut self.cur_tok, self.input.next_token())
    }

    // parses one expression
    pub fn parse_value(&mut self) -> Result<Value, ParseError>
    {
        match self.next_token() {
            Token::Lparen => self.parse_list(),
            Token::Rparen => Err(ParseError::UnexpectedRparen),
            Token::Quote => match self.parse_value() {
                Ok(val) => Ok(val.quote()),
                Err(ParseError::EndOfStream) => Err(ParseError::NoQuoteArg),
                Err(e) => Err(e),
            },
            Token::Number(val) => Ok(Value::Number(val)),
            Token::Ident(val) => Ok(Value::Symbol(Rc::new(val))),
            Token::String(val) => Ok(Value::String(Rc::new(val))),
            Token::Error(e) => Err(e),
            Token::End => Err(ParseError::EndOfStream),
        }
    }

    // parses the contents of a list
    fn parse_list(&mut self) -> Result<Value, ParseError>
    {
        let mut list = Vec::new();
        while self.cur_tok != Token::Rparen
        {
            if self.cur_tok == Token::End
            {
                return Err(ParseError::UnclosedList);
            }
            list.push(try!(self.parse_value()));
        }
        self.next_token();  // consume the ')'
        Ok(Value::List(List::from_de_iter(list.into_iter())))
    }

    // parses the entire chunk
    pub fn parse(mut self) -> Result<Vec<Value>, ParseError>
    {
        let mut out = Vec::new();
        loop
        {
            match self.parse_value() {
                Ok(val) => out.push(val),
                Err(ParseError::EndOfStream) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(out)
    }
}
