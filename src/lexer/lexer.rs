use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
enum Token {

    #[token("import")]
    Import,

    #[token("include")]
    Include,

    #[token("from")]
    From,

    #[token("let")]
    Let,

    #[token("var")]
    Var,

    #[token("enum")]
    Enum,

    #[token("model")]
    Model,

    #[token("type")]
    Type,

    #[token("function")]
    Function,

    #[token("struct")]
    Struct,

    #[token("config")]
    Config,

    #[token("native")]
    Native,

    #[token("where")]
    Where,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("%")]
    Percent,

    #[token("^")]
    Caret,

    #[token("&")]
    Ampersand,

    #[token("|")]
    Pipe,

    #[token("!")]
    Bang,

    #[token("?")]
    Question,

    #[token(".")]
    Period,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token("{")]
    BraceOpen,

    #[token("}")]
    BraceClose,

    #[token("(")]
    ParenOpen,

    #[token(")")]
    ParenClose,

    #[token("[")]
    BracketOpen,

    #[token("]")]
    BracketClose,

    #[token("<")]
    AngleOpen,

    #[token(">")]
    AngleClose,

    #[token("=")]
    Equal,

    #[regex("[a-zA-Z][a-zA-Z0-9_-]*")]
    Identifier,

    #[regex("/.+/")]
    Regex,

    #[regex(r#""(?:[^"]|\\")*""#)]
    String,

    #[regex("//[^\n]*")]
    LineComment,
}

fn main() {
    let mut lex = Token::lexer(r#"
        import { a } from "abc"
        model User {
            id: String,
            name: String,
            email: String,
            password: String,
            created_at: String,
            updated_at: String,
        }
        let a = "abc def ghi \" qqq"
    "#);
    for (token, span) in lex.spanned() {
        if let Ok(token) = token {
            println!("{:?} - {:?}", token, span);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;
    use super::*;

    #[test]
    fn token() {
        let start = SystemTime::now();
        main();
        // for x in 1..10000 {
        //     main()
        // }
        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        println!("duration {:?}", duration);
    }
}