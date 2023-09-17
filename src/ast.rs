use crate::token::Token;



pub type Program = Vec<Statement>;

pub type ExpToken = Token;
pub type Identifier = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement{
    SelectStatement{
        counters: Option<ExpToken>,
        quantity: Option<ExpToken>, //TOP(10) or TAIL(10)
        from: ExpToken,
        target: Identifier,
        condition: Option<Expression> //where line LIKE '%Wewew%'
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression{
    Where{
        conditional: Option<Token>, //conditional one of LIKE or NOTLIKE or REGEX
        target: Identifier,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
   
    Equals,
    NotEquals,
    Like,
    NotLike,
}

impl Op {
    pub fn token(token: Token) -> Self {
        match token {
            Token::Like => Self::Like,
            Token::NotLike => Self::NotLike,
            Token::NotEquals => Self::NotEquals,
            _ => unreachable!("{:?}", token)
        }
    }
}