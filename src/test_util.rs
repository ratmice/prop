use crate::codespan;
use crate::error::*;
use crate::lex;
use crate::parser;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

pub fn do_test<'a>(sources: &[&'a str]) -> Result<(), Vec<(&'a str, Error<'a>)>> {
    let (_pass, fail): (Vec<_>, Vec<_>) = sources
        .iter()
        .enumerate()
        .map(|(index, s)| {
            (
                index,
                parser::propParser::new().parse(lex::Tokens::from_string(s)),
            )
        })
        .partition(|(_, r)| r.is_ok());
    if fail.is_empty() {
        Ok(())
    } else {
        let errors: Vec<_> = fail
            .into_iter()
            .map(|(index, r)| (sources[index], r.unwrap_err()))
            .collect();
        Err(errors)
    }
}
// FIXME these 2 and print_errors can involve less duplication of slightly different code
// The difference: stdout vs stderr and ColorChoice::Never vs structopt which
// causes problems with cargo test implicit arguments.
pub fn expect_success<'a>(result: Result<(), Vec<(&'a str, Error<'a>)>>) -> Result<(), MainError> {
    match result {
        Ok(()) => Ok(()),
        Err(e) => {
            for (source, error) in e.iter() {
                let writer = StandardStream::stderr(ColorChoice::Never);
                let config = codespan_reporting::term::Config::default();
                let (files, diagnostic) = codespan::codespan("foo", source, error);

                eprintln!("capture stderr?");
                println!("capture stdout?");
                term::emit(&mut writer.lock(), &config, &files, &diagnostic)?;
            }
            Err(MainError::SomethingWentAwryAndStuffWasPrinted)
        }
    }
}

pub fn expect_fail<'a>(result: Result<(), Vec<(&'a str, Error<'a>)>>) -> Result<(), MainError> {
    match result {
        Ok(()) => {
            eprintln!("Did not get the failure that we expect");
            Err(MainError::SomethingWentAwryAndStuffWasPrinted)
        }

        Err(e) => {
            for (source, error) in e.iter() {
                let writer = StandardStream::stderr(ColorChoice::Never);
                let config = codespan_reporting::term::Config::default();
                let (files, diagnostic) = codespan::codespan("foo", source, error);

                term::emit(&mut writer.lock(), &config, &files, &diagnostic)?;
            }
            Ok(())
        }
    }
}
