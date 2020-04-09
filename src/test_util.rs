use crate::codespan;
use crate::error::*;
use crate::lex;
use crate::parser;
use codespan_reporting::term;
use logos::Logos;

pub fn do_test<'a>(sources: &[&'a str]) -> Result<(), Vec<(&'a str, Error<'a>)>> {
    let (_pass, fail): (Vec<_>, Vec<_>) = sources
        .iter()
        .enumerate()
        .map(|(index, s)| {
            (index, {
                parser::propParser::new().parse(lex::Token::lexer(&s).spanned().map(|(t, r)| {
                    if t == lex::Token::LexError {
                        Err(r.start)
                    } else {
                        Ok((r.start, t, r.end))
                    }
                }))
            })
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

pub fn expect_success<'a>(result: Result<(), Vec<(&'a str, Error<'a>)>>) -> Result<(), MainError> {
    match result {
        Ok(()) => Ok(()),
        Err(e) => {
            for (source, error) in e.iter() {
                let mut writer = codespan_reporting::term::termcolor::Buffer::no_color();
                let config = codespan_reporting::term::Config::default();
                let (files, diagnostic) = codespan::codespan("foo", source, error);

                term::emit(&mut writer, &config, &files, &diagnostic)?;
                eprintln!("{}", std::str::from_utf8(writer.as_slice()).unwrap())
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
                let mut writer = codespan_reporting::term::termcolor::Buffer::no_color();
                let config = codespan_reporting::term::Config::default();
                let (files, diagnostic) = codespan::codespan("foo", source, error);

                term::emit(&mut writer, &config, &files, &diagnostic)?;
                eprintln!("{}", std::str::from_utf8(writer.as_slice()).unwrap())
            }
            Ok(())
        }
    }
}
