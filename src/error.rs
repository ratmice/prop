use crate::lex;
pub type Error<'a> = lalrpop_util::ParseError<usize, lex::Token<'a>, lex::LexicalError>;

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
