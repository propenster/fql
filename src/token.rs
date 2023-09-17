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
    #[token("CREATE", ignore(ascii_case))] //create "~/home/files/new_file.txt"
    Create,
    #[token("fn")]
    Fn,
    #[token("SELECT", ignore(ascii_case))]
    Select,
    #[token("COUNTC", ignore(ascii_case))]
    CountC, //select countc * from "file.txt"
    #[token("COUNTL", ignore(ascii_case))]
    CountL,
    #[token("COUNTW", ignore(ascii_case))]
    CountW,
    #[token("*")]
    Star,
    #[token("FROM", ignore(ascii_case))]
    From,


    #[regex(r"(?i)(Top)\(\d+\)", transform_top_and_tail, ignore(ascii_case))]
    Top(String),

    #[regex(r"(?i)(Tail)\(\d+\)", transform_top_and_tail, ignore(ascii_case))]
    Tail(String),


    #[token("WHERE", ignore(ascii_case))]
    Where,
    #[token(">")]
    Export,
   



    // #[regex(r"[a-zA-Z_?]+", to_string)]
    // Identifier(String),
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
    #[token(r"LIKE", ignore(ascii_case))]
    Like,
    #[token(r"NOTLIKE", ignore(ascii_case))]
    NotLike,
    #[token("REGEX", ignore(ascii_case))]
    Regex,
    
    Eof,

    #[error]
    #[regex(r"--[^\n]*", logos::skip)]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,

}

impl Into<String> for Token{
    fn into(self) -> String {
        match self{
            //Token::Identifier(s) => s,
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
    fn it_can_recognise_select_statements(){
        let mut lexer = Token::lexer(r##"SELECT * FROM "C:\temp\workflow2.json";"##);
        assert_eq!(lexer.next(), Some(Token::Select));
        assert_eq!(lexer.next(), Some(Token::Star));
        assert_eq!(lexer.next(), Some(Token::From));
        assert_eq!(lexer.next(), Some(Token::Strings(r"C:\temp\workflow2.json".to_owned())));
        assert_eq!(lexer.next(), Some(Token::Eos));
    }
    #[test]
    fn it_can_recognise_select_with_where_like_condition(){
        let mut lexer = Token::lexer(r##"SELECT * FROM "C:\temp\workflow2.json" WHERE LIKE "taskid";"##);
        assert_eq!(lexer.next(), Some(Token::Select));
        assert_eq!(lexer.next(), Some(Token::Star));
        assert_eq!(lexer.next(), Some(Token::From));
        assert_eq!(lexer.next(), Some(Token::Strings(r"C:\temp\workflow2.json".to_owned())));
        assert_eq!(lexer.next(), Some(Token::Where));
        assert_eq!(lexer.next(), Some(Token::Like));
        assert_eq!(lexer.next(), Some(Token::Strings(r"taskid".to_owned())));
        assert_eq!(lexer.next(), Some(Token::Eos));
    }
    #[test]
    fn it_can_recognise_case_insensitive(){
        let mut lexer = Token::lexer(r##"select * fRom "C:\temp\workflow2.json" wHeRe LiKe "taskid";"##);
        assert_eq!(lexer.next(), Some(Token::Select));
        assert_eq!(lexer.next(), Some(Token::Star));
        assert_eq!(lexer.next(), Some(Token::From));
        assert_eq!(lexer.next(), Some(Token::Strings(r"C:\temp\workflow2.json".to_owned())));
        assert_eq!(lexer.next(), Some(Token::Where));
        assert_eq!(lexer.next(), Some(Token::Like));
        assert_eq!(lexer.next(), Some(Token::Strings(r"taskid".to_owned())));
        assert_eq!(lexer.next(), Some(Token::Eos));
    }
    #[test]
    fn it_can_recognise_select_with_where_not_like_condition(){
        let mut lexer = Token::lexer(r##"SELECT * FROM "C:\temp\workflow2.json" WHERE NOTLIKE "taskid";"##);
        assert_eq!(lexer.next(), Some(Token::Select));
        assert_eq!(lexer.next(), Some(Token::Star));
        assert_eq!(lexer.next(), Some(Token::From));
        assert_eq!(lexer.next(), Some(Token::Strings(r"C:\temp\workflow2.json".to_owned())));
        assert_eq!(lexer.next(), Some(Token::Where));
        assert_eq!(lexer.next(), Some(Token::NotLike));
        assert_eq!(lexer.next(), Some(Token::Strings(r"taskid".to_owned())));
        assert_eq!(lexer.next(), Some(Token::Eos));
    }
    #[test]
    fn it_can_recognise_select_with_count_statements(){
        let mut lexer = Token::lexer(r##"SELECT COUNTC * FROM "C:\temp\workflow2.json"; SELECT COUNTL * FROM "C:\temp\workflow2.json"; SELECT COUNTW * FROM "C:\temp\workflow2.json"; "##);
        //COUNTC
        assert_eq!(lexer.next(), Some(Token::Select));
        assert_eq!(lexer.next(), Some(Token::CountC));
        assert_eq!(lexer.next(), Some(Token::Star));
        assert_eq!(lexer.next(), Some(Token::From));
        assert_eq!(lexer.next(), Some(Token::Strings(r"C:\temp\workflow2.json".to_owned())));
        assert_eq!(lexer.next(), Some(Token::Eos));
        //countL
        assert_eq!(lexer.next(), Some(Token::Select));
        assert_eq!(lexer.next(), Some(Token::CountL));
        assert_eq!(lexer.next(), Some(Token::Star));
        assert_eq!(lexer.next(), Some(Token::From));
        assert_eq!(lexer.next(), Some(Token::Strings(r"C:\temp\workflow2.json".to_owned())));
        assert_eq!(lexer.next(), Some(Token::Eos));
         //countW
         assert_eq!(lexer.next(), Some(Token::Select));
         assert_eq!(lexer.next(), Some(Token::CountW));
         assert_eq!(lexer.next(), Some(Token::Star));
         assert_eq!(lexer.next(), Some(Token::From));
         assert_eq!(lexer.next(), Some(Token::Strings(r"C:\temp\workflow2.json".to_owned())));
         assert_eq!(lexer.next(), Some(Token::Eos));
    }


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

    // #[test]
    // fn it_can_recognise_identifiers(){
    //     //let mut lexer = Token::lexer("~/home/files/file.json hello_world C:\\lib\\file.txt");
    //     let mut lexer = Token::lexer("hello_world name age identifier salary");

    //     assert_eq!(lexer.next(), Some(Token::Identifier("hello_world".to_owned())));
    //     assert_eq!(lexer.next(), Some(Token::Identifier("name".to_owned())));
    //     assert_eq!(lexer.next(), Some(Token::Identifier("age".to_owned())));
    //     assert_eq!(lexer.next(), Some(Token::Identifier("identifier".to_owned())));

    //     assert_eq!(lexer.next(), Some(Token::Identifier("salary".to_owned())));



    // }

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
        assert_eq!(lexer.next(), Some(Token::Strings((r"testing".to_owned()))));

        assert_eq!(lexer.next(), Some(Token::Strings((r#"testing with \""#.to_owned()))));
        assert_eq!(lexer.next(), Some(Token::Strings((r#"testing \n"#.to_owned()))));
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