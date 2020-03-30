mod ast;
mod codespan;
mod error;
mod lex;
mod token_wrap;

#[cfg(test)]
mod test_util;
#[cfg(test)]
mod test;

use codespan_reporting::term::termcolor::StandardStream;
use codespan_reporting::term::{self, ColorArg};
use error::*;
use token_wrap::*;
use std::io::Read;
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
#[structopt(name = "prop")]
pub struct Opts {
    /// Configure coloring of output
    #[structopt(
        long = "color",
        default_value = "auto",
        possible_values = ColorArg::VARIANTS,
        case_insensitive = true,
    )]
    pub color: ColorArg,
}

mod parser {
    // Hack to avoid clippy lints in generated code.
    #![allow(clippy::all)]
    use lalrpop_util::lalrpop_mod;
    lalrpop_mod!(prop);
    pub use prop::*;
}

fn print_errors<'a>(result: Result<(), Vec<(&'a str, Error<'a>)>>) -> Result<(), MainError> {
    match result {
        Ok(()) => Ok(()),
        Err(e) => {
            for (source, error) in e.iter() {
                let opts = Opts::from_args();
                let writer = StandardStream::stderr(opts.color.into());
                let config = codespan_reporting::term::Config::default();
                let (files, diagnostic) = codespan::codespan("foo", source, error);

                term::emit(&mut writer.lock(), &config, &files, &diagnostic)?;
            }
            Err(MainError::SomethingWentAwryAndStuffWasPrinted)
        }
    }
}

fn main() -> Result<(), MainError> {
    let mut buf = std::io::BufReader::new(std::io::stdin());
    let mut s = Box::new(String::new());

    // Not really how i'd like this to be.
    buf.read_to_string(&mut s)?;
    let lexer = Tokens::from_string(&s);
    let parse_result = parser::propParser::new().parse(lexer);

    match parse_result {
        Err(e) => {
            // FIXME terrible_vec isn't needed anymore since
            // I moved stuff to test_util.
            let mut terrible_vec = Vec::new();
            terrible_vec.push((s.as_str(), e));
            print_errors(Err(terrible_vec))?;
            return Err(MainError::SomethingWentAwryAndStuffWasPrinted);
        }
        Ok(exprs) => {
            for bind in exprs.iter() {
                println!("{}", bind);
            }
        }
    }
    Ok(())
}
