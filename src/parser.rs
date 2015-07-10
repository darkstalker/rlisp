use std::mem;
use data::{Token, Value, ParseError};
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
        Parser{ cur_tok: tokens.next(), input: tokens }
    }

    // consumes the current token and pulls a new one
    fn next_token(&mut self) -> Token
    {
        mem::replace(&mut self.cur_tok, self.input.next())
    }

    // parses one expression
    pub fn parse_value(&mut self) -> Result<Value, ParseError>
    {
        match self.next_token() {
            Token::Lparen => self.parse_list(),
            Token::Rparen => Err(ParseError::UnexpectedRparen),
            Token::Quote => Ok(Value::quote(try!(self.parse_value()))),
            Token::Number(val) => Ok(Value::Number(val)),
            Token::Ident(val) => Ok(Value::Ident(val)),
            Token::String(val) => Ok(Value::String(val)),
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
        Ok(Value::List(list))
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
