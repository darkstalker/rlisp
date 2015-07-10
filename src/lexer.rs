use std::str::Chars;
use data::{Token, ParseError};

// take from the char after the \" to the next \"
fn extract_string(input: &mut Chars) -> Result<String, ParseError>
{
    let mut buf = String::new();
    let mut escape = false;
    while let Some(chr) = input.next()
    {
        if escape
        {
            buf.push(match chr {
                'n' => '\n',
                't' => '\t',
                other => other,
            });
            escape = false;
        }
        else
        {
            match chr {
                '\\' => escape = true,
                '"' => return Ok(buf),
                other => buf.push(other),
            }
        }
    }
    Err(ParseError::UnclosedString)
}

// take from the char after the first to the next separator
fn extract_ident(input: &mut Chars, first: char) -> String
{
    let mut buf = first.to_string();
    let mut copied = input.clone();
    while let Some(chr) = copied.next()
    {
        match chr {
            ' ' | '(' | ')' | '\n' | '\t' => break,
            other => { buf.push(other); input.next(); },
        }
    }
    buf
}

// takes a single token from the input stream
fn extract_token(input: &mut Chars) -> Token
{
    match input.skip_while(|c| *c == ' ' || *c == '\n' || *c == '\t').next() {
        Some(chr) => match chr {
            '(' => Token::Lparen,
            ')' => Token::Rparen,
            '\'' => Token::Quote,
            '"' => match extract_string(input) {
                Ok(val) => Token::String(val),
                Err(e) => Token::Error(e),
            },
            '-' => {
                let ident = extract_ident(input, '-');
                match ident.parse() {
                    Ok(val) => Token::Number(val),
                    Err(_) => Token::Ident(ident),
                }
            },
            dig @ '0'...'9' => match extract_ident(input, dig).parse() {
                Ok(val) => Token::Number(val),
                Err(_) => Token::Error(ParseError::InvalidNumber),
            },
            other => Token::Ident(extract_ident(input, other)),
        },
        None => Token::End,
    }
}

#[derive(Clone)]
pub struct Tokenizer<'a>(Chars<'a>);

impl<'a> Tokenizer<'a>
{
    pub fn new(text: &str) -> Tokenizer
    {
        Tokenizer(text.chars())
    }

    pub fn next(&mut self) -> Token
    {
        extract_token(&mut self.0)
    }
}
