use logos::Logos;
use rowan::SmolStr;
use std::ops::Range;

impl From<LexToken> for rowan::SyntaxKind {
    fn from(kind: LexToken) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lang {}
impl rowan::Language for Lang {
    type Kind = LexToken;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= LexToken::Root as u16);
        unsafe { std::mem::transmute::<u16, LexToken>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

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
#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum LexToken {
    // Unfortunately Logos derive doesn't let us use
    // I presume it might be ensuring that itself, and actively stopping
    // people from doing e.e. EOF = 1 and messing up its internal indexing.
    // I'm not certain yet whether this is something that can be relied on.
    //
    // EOF = 0,
    #[end]
    EOF,

    #[token = "."]
    Dot,

    #[token = ":="]
    #[token = "≔"]
    Def,

    // FIXME pick only one of these.
    // perhaps we can do some proc-macro stuff to choose from a map file.
    // having multiple tokens here impedes it from being a single bijection
    // so we either need to pick one, or build a way to deal with multiple bijections.
    //
    // I find -> harder to type even though its standard, readable, and shorter.
    // So choosing \to for now, but that is not a good argument for it
    // being the default.
    //
    // #[token = "->"]
    #[token = r"\to"]
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

    // Name ↔ Name
    #[regex = r"[a-zA-Z][_a-zA-Z0-9]*"]
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
    //
    // FancyNameAscii ↔ FancyNameUnicode
    #[regex = r"[\\][a-zA-Z][_a-zA-Z0-9]*"]
    FancyNameAscii,
    #[regex = r"[a-zA-Z\p{Greek}\x{1d49c}-\x{1d59f}\x{2100}-\x{214f}][_a-zA-Z0-9\x{207f}-\x{2089}\x{2090}-\x{209c}\x{1d62}-\x{1d6a}]*"]
    FancyNameUnicode,

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

    Binder,
    Root,
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

#[derive(Debug)]
pub struct LexicalError(pub Range<usize>);

impl std::fmt::Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "lexical error at {:?}", self.0)
    }
}

pub struct Tokens<'a>(logos::Lexer<LexToken, &'a str>);
pub struct TokensRowan<'a>(logos::Lexer<LexToken, &'a str>);

#[derive(Debug, Clone)]
pub enum RowanToken {
  Token{token: LexToken, string: SmolStr}
}

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

impl<'a> Tokens<'a> {
    pub fn from_string(source: &'a str) -> Tokens<'a> {
        Tokens(LexToken::lexer(source))
    }
}

impl<'a> TokensRowan<'a> {
    pub fn from_string(source: &'a str) -> TokensRowan<'a> {
        TokensRowan(LexToken::lexer(source))
    }
}

impl<'a> Iterator for TokensRowan<'a> {
    type Item = Spanned<RowanToken, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        let lex = &mut self.0;
        let range = lex.range();
        let tok = lex.token;
        let tok = if LexToken::EOF == tok {
            None
        } else {
            Some(Ok((
                range.start,
                RowanToken::Token{token: lex.token, string: lex.slice().into()},
                range.end,
            )))
        };
        lex.advance();
        tok
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
                LexToken::Whitespace | LexToken::Comment => lex.advance(),
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
                LexToken::Root | LexToken::Binder => unreachable!(),
            }
        };
        lex.advance();
        Some(token)
    }
}
