use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex(r"[ \t\n\f]", priority = 0)]
    Whitespace,
    #[token(";")]
    SemiColon,
    #[token(".")]
    Period,
    #[token("::")]
    DoubleColon,
    #[token(":")]
    Colon,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftCurly,
    #[token("}")]
    RightCurly,
    #[regex("[a-zA-Z_]+")]
    Ident,
    #[regex(r"//[^/]([^\n]*)\n")]
    Comment,
    #[regex(r"/\*[^\*][^\*/]*\*/")]
    BlockComment,
    #[regex(r"///[^\n]*")]
    DocComment,
    #[regex(r"/\*\*[^\*/]*\*/")]
    DocBlockComment
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call() {
        let tok: Result<Vec<Token>, ()> = Token::lexer("a::C.d(e); /* i think */ //this is a cool function
/** doc comment */").collect();
        assert_eq!(tok.unwrap(), vec![
            Token::Ident,
            Token::DoubleColon,
            Token::Ident,
            Token::Period,
            Token::Ident,
            Token::LeftParen,
            Token::Ident,
            Token::RightParen,
            Token::SemiColon,
            Token::Whitespace,
            Token::BlockComment,
            Token::Whitespace,
            Token::Comment,
            Token::DocBlockComment
        ])
    }
}