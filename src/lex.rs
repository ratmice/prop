use logos::Logos;
use std::ops::Range;

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

// Notably absent from the above, present in the below are
// Whitespace, EOF, LexError
#[derive(Logos, Debug)]
enum _Token_ {
    #[end]
    EOF,

    #[token = "."]
    Dot,

    #[token = ":="]
    #[token = "≔"]
    Def,

    #[token = r"\to"]
    #[token = "->"]
    #[token = "→"]
    Arrow,

    #[token = r"\iff"]
    #[token = "↔"]
    Iff,

    #[token = r"\neg"]
    #[token = "¬"]
    Neg,

    // note not the greek lambda character but the coptic character.
    #[token = r"\lam"]
    #[token = "ⲗ"]
    Abs,

    #[token = r"\and"]
    #[token = "∧"] // Not ^
    Conj,

    #[token = r"\or"]
    #[token = "∨"] // Not v compare: ∨v
    Disj,

    #[token = r"\top"]
    #[token = "⊤"]
    Top,

    #[token = r"\bot"]
    #[token = "⊥"]
    Bot,

    #[token = "("]
    LParen,
    #[token = ")"]
    RParen,

    #[regex = r"[\\a-zA-Z][_a-zA-Z0-9]*"]
    Name,

    // Since this uses Coptic letters for keywords all greek letters can be used as variable names.
    // Variables can start with a slash character, a greek/math alphanumeric symbol,
    // and ascii letters numbers, and subscripts (TODO superscripts)
    //
    // The above are commented out because of their affect on compile times...
    // In some cases they cause the proc-macro to overflow the stack
    // Essentially you should think of this like:
    //
    // [\\a-zA-Z][a-zA-Z0-9]
    //
    // I.e. the normal thing + a \.
    // then it just adds a bunch of math letters to the first [...]
    // and subscripts to the second.
    //
    // \x{2100}-\x{214f} Letter-like symbols.
    // A subset of the Mathematical Alphanumeric symbols -- Mathematical Script, Fractur, Double-Struck
    // https://unicode.org/charts/PDF/U1D400.pdf
    // \x{1d49c}-\x{1d59f}
    //
    // Subscripts:
    // \x{2080}-\x{2089}
    // \x{2090}-\x{209c}
    // \x{1d62}-\x{1d6a}
    #[regex = r"[\\a-zA-Z\p{Greek}\x{1d49c}-\x{1d59f}\x{2100}-\x{214f}][_a-zA-Z0-9\x{207f}-\x{2089}\x{2090}-\x{209c}\x{1d62}-\x{1d6a}]*"]
    FancyName,

    #[token = ":"]
    Colon,

    #[token = ";"]
    Semi,

    #[token = r"\p{Whitespace}"]
    Whitespace,

    #[regex = r"#.*\n"]
    Comment,

    #[error]
    LexError,
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

pub struct Tokens<'a>(logos::Lexer<_Token_, &'a str>);
pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

impl<'a> Tokens<'a> {
    pub fn from_string(source: &'a str) -> Tokens<'a> {
        Tokens(_Token_::lexer(source))
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
                _Token_::Whitespace | _Token_::Comment => lex.advance(),
                _Token_::EOF => return None,
                _Token_::LexError => break Err(LexicalError(range)),
                #[rustfmt::skip]
                _Token_::Name      => break ok(Token::Name(lex.slice())),
                _Token_::FancyName => break ok(Token::Name(lex.slice())),
                // And the rest are all unary members
                _Token_::Dot => break ok(Token::Dot),
                _Token_::Abs => break ok(Token::Abs),
                _Token_::Bot => break ok(Token::Bot),
                _Token_::Top => break ok(Token::Top),
                _Token_::Neg => break ok(Token::Neg),
                _Token_::Iff => break ok(Token::Iff),
                _Token_::Def => break ok(Token::Def),
                _Token_::Disj => break ok(Token::Disj),
                _Token_::Conj => break ok(Token::Conj),
                _Token_::Semi => break ok(Token::Semi),
                _Token_::Arrow => break ok(Token::Arrow),
                _Token_::Colon => break ok(Token::Colon),
                _Token_::LParen => break ok(Token::LParen),
                _Token_::RParen => break ok(Token::RParen),
            }
        };
        lex.advance();
        Some(token)
    }
}
