use std::slice::Iter;

use crate::ast::*;
use crate::token::Token;
use thiserror::Error;

pub fn parse(tokens: Vec<Token>) -> Result<Program, ParseError> {
    let mut parser = Parser::new(tokens.iter());
    parser.read();
    parser.read();

    let mut program: Program = Vec::new();

    while let Some(statement) = parser.next()? {
        program.push(statement);
    }
    println!("We have {} QUERIES to run", program.len());
    Ok(program)
}

pub struct Parser<'a> {
    tokens: Iter<'a, Token>,
    current: Token,
    peek: Token,
}

impl<'a> Parser<'a> {
    fn new(tokens: Iter<'a, Token>) -> Self {
        Self {
            tokens: tokens,
            current: Token::Eof,
            peek: Token::Eof,
        }
    }

    fn next(&mut self) -> Result<Option<Statement>, ParseError> {
        println!("Current before next {:?}", self.current);
        if self.current_is(Token::Eos) {
            self.read();
            self.read();
        }
        println!(
            "After EOS self.read we now have a new token which is >>> {:?}",
            self.current
        );
        if self.current == Token::Eof {
            return Ok(None);
        }
        Ok(Some(self.parse_statement()?))
    }
    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.current {
            Token::Select => self.parse_select_statement(),
            _ => Err(ParseError::UnexpectedToken(self.current.clone())),
        }
    }

    fn parse_select_statement(&mut self) -> Result<Statement, ParseError> {
        self.expect_token_and_read(Token::Select)?;

        let counters = if self.current_is(Token::CountC){
            let token = self.expect_token_and_read(Token::CountC)?;
            Some(token)
        }else if self.current_is(Token::CountL){
            let token = self.expect_token_and_read(Token::CountL)?;
            Some(token)
        }else if self.current_is(Token::CountW){
            let token = self.expect_token_and_read(Token::CountW)?;
            Some(token)
        }else{
            None
        };

        let quantity = if self.current_is(Token::Top("".to_owned())) {
            self.expect_token_and_read(Token::Top("".to_owned()))?
        } else if self.current_is(Token::Star) {
            self.expect_token_and_read(Token::Star)?
        } else {
            self.expect_token_and_read(Token::Tail("".to_owned()))?
        };

        let from = self.expect_token_and_read(Token::From)?;

        let target: String = self.expect_identifier_and_read()?.into();

        // Do we have Conditional Expression after TARGET "../filePath.txt"?
        let condition_expression: Option<Expression> = if self.current_is(Token::Where) {
            self.expect_token_and_read(Token::Where)?;
            //after reading WHERE, next should be a conditional followed by a STRING LITERAL of the Search TERM...
            if !self.current_is(Token::Like) || !self.current_is(Token::NotLike){
                return Err(ParseError::UnexpectedToken(self.current.clone()))
            }
            let conditional = match self.current{
                Token::Like => Token::Like,
                Token::NotLike => Token::NotLike,
                _ => unreachable!()
            };
            //move to the next token...
            // if self.peek != Token::Strings("".to_owned()){
            //     return Err(ParseError::UnexpectedToken(self.current.clone()))
            // }
            let target_search_string: String  = match self.expect_identifier_and_read(){
                Ok(s) => s.into(),
                Err(e) => return Err(ParseError::UnexpectedToken(self.current.clone()))
            };

            Some(Expression::Where { conditional: Some(conditional), target: target_search_string })

        } else {
            None
        };

        Ok(Statement::SelectStatement {
            counters: counters,
            quantity: Some(quantity),
            from: from,
            target: target,
            condition: condition_expression,
        })
    }
    fn expect_token(&mut self, token: Token) -> Result<Token, ParseError> {
        let c = self.current.clone();
        println!("current Token: {:?}", c);
        if self.current_is(token) {
            Ok(self.current.clone())
        } else {
            Err(ParseError::UnexpectedToken(self.current.clone()))
        }
    }

    fn expect_token_and_read(&mut self, token: Token) -> Result<Token, ParseError> {
        let result = self.expect_token(token)?;

        println!("Result Token: {:?}", result);

        self.read();

        Ok(result)
    }

    fn expect_identifier_and_read(&mut self) -> Result<Token, ParseError> {
        self.expect_token_and_read(Token::Strings("".to_string()))
    }
    fn expect_top_token_and_read(&mut self) -> Result<Token, ParseError> {
        self.expect_token_and_read(Token::Top("".to_string()))
    }
    fn expect_tail_token_and_read(&mut self) -> Result<Token, ParseError> {
        self.expect_token_and_read(Token::Tail("".to_string()))
    }

    fn current_is(&self, token: Token) -> bool {
        std::mem::discriminant(&self.current) == std::mem::discriminant(&token)
    }

    fn read(&mut self) {
        self.current = self.peek.clone();
        self.peek = if let Some(token) = self.tokens.next() {
            token.clone()
        } else {
            Token::Eof
        };
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected token {0:?}.")]
    UnexpectedToken(Token),
}
