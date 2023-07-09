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
    #[token(",")]
    Comma,
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
    #[token("!")]
    Not,

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

    #[regex(r"[1-9][_1-9]*")]
    IntegerLiteral,
    #[regex(r#""(?:[^"]|\\")*""#)]
    StringLiteral,

    // Keyword Tokens
    #[token("fn")]
    FunctionKeyword,
    #[token("async")]
    AsyncKeyword,
    #[token("let")]
    LetKeyword,
    #[token("mut")]
    MutKeyword
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
pub struct TokenStream<'a> {
    val: logos::Lexer<'a, LogosToken>
}


pub fn lex<'a>(input: &'a str) -> TokenStream<'a> {
    let t = LogosToken::lexer(input);
    let ts = transform::transform(t);
    ts
}