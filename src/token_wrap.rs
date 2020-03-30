use crate::lex;
use std::ops::Range;
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
    #[rustfmt::skip]
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
            Token::Name(s)      => write!(f, "{}", s),
        }
    }
}

#[derive(Debug)]
pub struct LexicalError(pub Range<usize>);

impl std::fmt::Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "lexical error at {:?}", self.0)
    }
}

pub struct Tokens<'a>(logos::Lexer<lex::Token, &'a str>);
pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

impl<'a> Tokens<'a> {
    pub fn from_string(source: &'a str) -> Tokens<'a> {
        Tokens(lex::Token::lexer(source))
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
                lex::Token::Whitespace | lex::Token::Comment => lex.advance(),
                lex::Token::EOF => return None,
                lex::Token::LexError => break Err(LexicalError(range)),
                lex::Token::Name => break ok(Token::Name(lex.slice())),
                lex::Token::FancyNameAscii => break ok(Token::Name(lex.slice())),
                lex::Token::FancyNameUnicode => break ok(Token::Name(lex.slice())),
                // And the rest are all unary members
                lex::Token::Dot => break ok(Token::Dot),
                lex::Token::Abs => break ok(Token::Abs),
                lex::Token::Bot => break ok(Token::Bot),
                lex::Token::Top => break ok(Token::Top),
                lex::Token::Neg => break ok(Token::Neg),
                lex::Token::Iff => break ok(Token::Iff),
                lex::Token::Def => break ok(Token::Def),
                lex::Token::Disj => break ok(Token::Disj),
                lex::Token::Conj => break ok(Token::Conj),
                lex::Token::Semi => break ok(Token::Semi),
                lex::Token::Arrow => break ok(Token::Arrow),
                lex::Token::Colon => break ok(Token::Colon),
                lex::Token::LParen => break ok(Token::LParen),
                lex::Token::RParen => break ok(Token::RParen),
            }
        };
        lex.advance();
        Some(token)
    }
}
