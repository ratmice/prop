use crate::lex::Token;

pub type Error<'a> = lalrpop_util::ParseError<usize, Token<'a>, usize>;

#[derive(Debug)]
pub enum MainError {
    IO(std::io::Error),
    SomethingWentAwryAndStuffWasPrinted,
}

impl<'a> From<std::io::Error> for MainError {
    fn from(it: std::io::Error) -> Self {
        MainError::IO(it)
    }
}
