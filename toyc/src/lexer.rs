use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex(r"[ \t\n\f]", priority = 1000)]
    Whitespace,
    #[token(";")]
    SemiColon,

    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[regex("[a-zA-Z_]+", priority = 999)]
    Ident,
    #[regex(r"//[^\n]*", priority = 4)]
    #[regex(r"//\*([^\*])((?:[^\*]|\*[^/])*)\*//", priority = 1)]
    Comment,
    #[regex(r"//\*\*((?:[^*]|\*[^/])*)\*//", priority = 0)]
    DocComment
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comments() {
        assert_eq!(Token::lexer("/** hi . 7c huy */").next().unwrap().unwrap(), Token::DocComment);
        assert_eq!(Token::lexer("/* hi . 7c huy */").next().unwrap().unwrap(), Token::Comment);
    }
}