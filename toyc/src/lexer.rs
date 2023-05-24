use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex(r"[ \t\n\f]", priority = 0)]
    Whitespace,
    #[token(";")]
    SemiColon,

    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[regex("[a-zA-Z_]+", priority = 1)]
    Ident,
    #[regex(r"//[^/]([^\n]*)", priority = 2)]
    #[regex(r"/\*[^*]([\s\S]*)\*/", priority = 3)]
    Comment,
    #[regex(r"///([^\n]*)", priority = 4)]
    #[regex(r"/\*\*([\s\S]*)\*/", priority = 5)]
    DocComment
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comments() {
        let a: Vec<_> = Token::lexer("/** */").collect();
        println!("{:?}", a);
        assert_eq!(a[0].as_ref().unwrap(), &Token::DocComment);
        assert_eq!(Token::lexer("/* */").next().unwrap().unwrap(), Token::Comment);
    }
}