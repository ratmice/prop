use crate::lex::{LexToken, LexicalError, Spanned};
use logos::Logos;
use rowan;
pub struct Tokens<'a>(logos::Lexer<LexToken, &'a str>);

pub type SyntaxNode = rowan::SyntaxNode<Lang>;
// type SyntaxToken = rowan::SyntaxToken<Lang>;
// type SyntaxElement = rowan::SyntaxElement<Lang>;

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
pub enum TokenWrap {
    Token {
        token: LexToken,
        string: rowan::SmolStr,
    },
}

impl<'a> Tokens<'a> {
    pub fn from_string(source: &'a str) -> Tokens<'a> {
        Tokens(LexToken::lexer(source))
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Spanned<TokenWrap, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        let lex = &mut self.0;
        let range = lex.range();
        let tok = lex.token;
        let tok = if LexToken::EOF == tok {
            None
        } else {
            Some(Ok((
                range.start,
                TokenWrap::Token {
                    token: lex.token,
                    string: lex.slice().into(),
                },
                range.end,
            )))
        };
        lex.advance();
        tok
    }
}

#[cfg(test)]
use crate::{error, lex, rowan_parser, test_util};

#[test]
fn rowan_lex() -> Result<(), error::MainError> {
    let s = "trivial ≔ ⊤; X ≔ ⊤; Y := Z;";
    test_util::expect_success(test_util::do_test(&[s]))?;

    let tokens = Tokens::from_string(s);
    let tokens = tokens.map(|x| {
        println!("{:?}", x);
        x
    });

    let mut builder = rowan::GreenNodeBuilder::new();

    builder.start_node(lex::LexToken::Root.into());
    let parse_result = rowan_parser::propParser::new().parse(&mut builder, tokens);
    match parse_result {
        Ok(_) => (),
        Err(e) => {
            println!("{:?}", e);
            return Err(error::MainError::SomethingWentAwryAndStuffWasPrinted);
        }
    }
    builder.finish_node();
    Ok(())
}
