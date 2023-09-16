use logos::{Logos, Lexer};



pub fn generate(input: &str) -> Vec<Token>{
    Token::lexer(input).collect()
}

fn to_string(lex: &mut Lexer<Token>) -> Option<String>{
    //return the input string slice that matches the spec in Token enum
    let mut word = lex.slice().to_string();

    if word.starts_with('"') && word.ends_with("\""){
        word.remove(0);
        word.remove(word.len()-1);
    }
    
    Some(word)
}
fn transform_top_and_tail(lex: &mut Lexer<Token>) -> Option<String>{
    //return the input string slice that matches the spec in Token enum
    Some(lex.slice().to_string().to_uppercase())
}
fn to_float(lex: &mut Lexer<Token>) -> Option<f64>{
    Some(lex.slice().parse().ok()?)
}

#[derive(Debug, Clone, Logos, PartialEq)]
pub enum Token{
    #[token("fn")]
    Fn,
    #[token("SELECT")]
    Select,
    #[regex(r"(?i)COUNTC")]
    CountC, //select countc * from "file.txt"
    #[regex(r"(?i)COUNTL")]
    CountL,
    #[regex(r"(?i)COUNTW")]
    CountW,
    #[token("*")]
    Star,
    #[token("FROM")]
    From,


    #[regex(r"(?i)(Top)\(\d+\)", transform_top_and_tail)]
    Top(String),

    #[regex(r"(?i)(Tail)\(\d+\)", transform_top_and_tail)]
    Tail(String),


    #[token("WHERE")]
    Where,
    #[token(">")]
    Export,
    #[token("CREATE")] //create "~/home/files/new_file.txt"
    Create,




    #[regex(r"[a-zA-Z_?]+", to_string)]
    Identifier(String),
    #[regex(r"([0-9]+[.])?[0-9]+", to_float)]
    Number(f64),
    #[regex(r##""(?:[^"\\]|\\.)*""##, to_string)]
    Strings(String),
   

    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,


    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("/")]
    Slash,
    #[token("=")]
    Assign,


    #[token("true")]
    True,
    #[token("false")]
    False,
    
    #[token(";")]
    Eos,
    #[token("let")]
    Let,


    #[token("!=")]
    NotEquals,
    #[regex(r"(?i)LIKE")]
    Like,
    #[regex(r"(?i)NOTLIKE")]
    NotLike,
    
    Eof,

    #[error]
    #[regex(r"--[^\n]*", logos::skip)]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,

}

impl Into<String> for Token{
    fn into(self) -> String {
        match self{
            Token::Identifier(s) => s,
            Token::Strings(s) => s,
            Token::Top(s) => s,
            Token::Tail(s) => s,
            _ => unreachable!()
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;


    #[test]
    fn it_can_recognise_reserved_keywords(){
        let mut lexer = Token::lexer("fn true false SELECT FROM WHERE CREATE");

        assert_eq!(lexer.next(), Some(Token::Fn));
        assert_eq!(lexer.next(), Some(Token::True));
        assert_eq!(lexer.next(), Some(Token::False));
        assert_eq!(lexer.next(), Some(Token::Select));

        assert_eq!(lexer.next(), Some(Token::From));
        assert_eq!(lexer.next(), Some(Token::Where));
        assert_eq!(lexer.next(), Some(Token::Create));

    }

    #[test]
    fn it_can_recognise_identifiers(){
        //let mut lexer = Token::lexer("~/home/files/file.json hello_world C:\\lib\\file.txt");
        let mut lexer = Token::lexer("hello_world name age identifier salary");

        assert_eq!(lexer.next(), Some(Token::Identifier("hello_world".to_owned())));
        assert_eq!(lexer.next(), Some(Token::Identifier("name".to_owned())));
        assert_eq!(lexer.next(), Some(Token::Identifier("age".to_owned())));
        assert_eq!(lexer.next(), Some(Token::Identifier("identifier".to_owned())));

        assert_eq!(lexer.next(), Some(Token::Identifier("salary".to_owned())));



    }

    #[test]
    fn it_can_recognise_numbers(){
        let mut lexer = Token::lexer("12345 6789.98 2.5 4.51");
        assert_eq!(lexer.next(), Some(Token::Number(12345.0)));
        assert_eq!(lexer.next(), Some(Token::Number(6789.98)));
        assert_eq!(lexer.next(), Some(Token::Number(2.5)));
        assert_eq!(lexer.next(), Some(Token::Number(4.51)));
    }
    
    #[test]
    fn it_can_recognise_strings(){
        let mut lexer = Token::lexer(r##""testing" "testing with \"" "testing \n""##);
        assert_eq!(lexer.next(), Some(Token::Strings((r##""testing""##.to_owned()))));

        assert_eq!(lexer.next(), Some(Token::Strings((r##""testing with \"""##.to_owned()))));
        assert_eq!(lexer.next(), Some(Token::Strings((r##""testing \n""##.to_owned()))));
    }

    #[test]
    fn it_can_recognise_symbols(){
        let mut lexer = Token::lexer("( ) { } + - * /");
        assert_eq!(lexer.next(), Some(Token::LeftParen));
        assert_eq!(lexer.next(), Some(Token::RightParen));
        assert_eq!(lexer.next(), Some(Token::LeftBrace));
        assert_eq!(lexer.next(), Some(Token::RightBrace));

        assert_eq!(lexer.next(), Some(Token::Plus));
        assert_eq!(lexer.next(), Some(Token::Minus));
        assert_eq!(lexer.next(), Some(Token::Star));
        assert_eq!(lexer.next(), Some(Token::Slash));  

    }

 #[test]
 fn it_can_match_top_and_tail_lexis(){
    let mut lexer = Token::lexer(r##"Top(15) Top(100) Tail(15) Tail(100) tOp(12)"##);

    assert_eq!(lexer.next(), Some(Token::Top("TOP(15)".to_owned())));
    assert_eq!(lexer.next(), Some(Token::Top("TOP(100)".to_owned())));

    assert_eq!(lexer.next(), Some(Token::Tail("TAIL(15)".to_owned())));
    assert_eq!(lexer.next(), Some(Token::Tail("TAIL(100)".to_owned())));

    assert_eq!(lexer.next(), Some(Token::Top("TOP(12)".to_owned())));

 }




}