mod transform;

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum LogosToken {
    // Normal Tokens
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
    #[token("#[")]
    TagSquareBracketLeft,
    #[token("[")]
    SquareBracketLeft,
    #[token("]")]
    SquareBracketRight,
    #[regex("[a-zA-Z_]+")]
    Ident,
    #[regex(r"//[^/]([^\n]*)\n")]
    Comment,
    #[regex(r"/\*[^\*][^\*/]*\*/")]
    BlockComment,
    #[regex(r"///[^\n]*")]
    DocComment,
    #[regex(r"/\*\*[^\*/]*\*/")]
    DocBlockComment,

    // Keyword Tokens
    #[token("fn")]
    FunctionKeyword,
    #[token("async")]
    AsyncKeyword
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call() {
        let tok: Result<Vec<LogosToken>, ()> = LogosToken::lexer("a::C.d(e); /* i think */ //this is a cool function
/** doc comment */").collect();
        assert_eq!(tok.unwrap(), vec![
            LogosToken::Ident,
            LogosToken::DoubleColon,
            LogosToken::Ident,
            LogosToken::Period,
            LogosToken::Ident,
            LogosToken::LeftParen,
            LogosToken::Ident,
            LogosToken::RightParen,
            LogosToken::SemiColon,
            LogosToken::Whitespace,
            LogosToken::BlockComment,
            LogosToken::Whitespace,
            LogosToken::Comment,
            LogosToken::DocBlockComment
        ])
    }
}

pub struct Token {
    span: usize,
    tokentype: LogosToken,
    source: String
}

pub struct TokenStream(Vec<Token>);


pub fn lex(input: String) -> TokenStream {
    let t = LogosToken::lexer(input.as_str());
    let ts = transform::transform(t);
    ts
}