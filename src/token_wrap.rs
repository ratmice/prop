use crate::lex::{LexToken, LexicalError, Spanned};
use logos::Logos;

#[derive(Debug, Clone)]
pub enum Token<'a> {
    Dot,
    Semi,
    Colon,
    LParen,
    RParen,
    Bot,
    Top,
    Disj,
    Conj,
    Abs,
    Neg,
    Iff,
    Arrow,
    Def,
    Name(&'a str),
}

impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Dot => write!(f, "."),
            Token::Abs => write!(f, "ⲗ"),
            Token::Bot => write!(f, "⊥"),
            Token::Def => write!(f, "≔"),
            Token::Iff => write!(f, "↔"),
            Token::Neg => write!(f, "¬"),
            Token::Top => write!(f, "⊤"),
            Token::Conj => write!(f, "∧"),
            Token::Disj => write!(f, "∨"),
            Token::Semi => write!(f, ";"),
            Token::Arrow => write!(f, "→"),
            Token::Colon => write!(f, ":"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Name(s) => write!(f, "{}", s),
        }
    }
}

pub struct Tokens<'a>(logos::Lexer<LexToken, &'a str>);

impl<'a> Tokens<'a> {
    pub fn from_string(source: &'a str) -> Tokens<'a> {
        Tokens(LexToken::lexer(source))
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Spanned<Token<'a>, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        let lex = &mut self.0;
        let range = lex.range();
        let ok = |tok: Token<'a>| Ok((range.start, tok, range.end));
        let token = loop {
            match &lex.token {
               LexToken::Whitespace |LexToken::Comment => lex.advance(),
               LexToken::EOF => return None,
               LexToken::LexError => break Err(LexicalError(range)),
               LexToken::Name => break ok(Token::Name(lex.slice())),
               LexToken::FancyNameAscii => break ok(Token::Name(lex.slice())),
               LexToken::FancyNameUnicode => break ok(Token::Name(lex.slice())),
                // And the rest are all unary members
               LexToken::Dot => break ok(Token::Dot),
               LexToken::Abs => break ok(Token::Abs),
               LexToken::Bot => break ok(Token::Bot),
               LexToken::Top => break ok(Token::Top),
               LexToken::Neg => break ok(Token::Neg),
               LexToken::Iff => break ok(Token::Iff),
               LexToken::Def => break ok(Token::Def),
               LexToken::Disj => break ok(Token::Disj),
               LexToken::Conj => break ok(Token::Conj),
               LexToken::Semi => break ok(Token::Semi),
               LexToken::Arrow => break ok(Token::Arrow),
               LexToken::Colon => break ok(Token::Colon),
               LexToken::LParen => break ok(Token::LParen),
               LexToken::RParen => break ok(Token::RParen),
               LexToken::Root |LexToken::Binder => unreachable!(),
            }
        };
        lex.advance();
        Some(token)
    }
}
