use crate::error::*;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;

// needs much work.
pub fn codespan<'a>(
    filename: &'a str,
    data: &'a str,
    error: &Error<'a>,
) -> (SimpleFiles<&'a str, &'a str>, Diagnostic<usize>) {
    use lalrpop_util::ParseError::*;

    let mut files = SimpleFiles::new();
    let file_id = files.add(filename, data);

    // fn join_expected(v: &Vec<String>) -> String {
    let join_expected = |v: &Vec<String>| {
        let len = v.len();
        //  let get_string = |v: &Vec<String>, i: usize| v[i].to_string();
        let get_string = |v: &Vec<String>, i: usize| unsafe { v.get_unchecked(i).to_string() };
        if len == 0 {
            String::new()
        } else if len == 1 {
            get_string(v, 0)
        } else {
            format!(
                "{} or {}",
                v[0..(len - 2)].join(", "),
                get_string(v, len - 1)
            )
        }
    };

    let fmt_expected = |expected| format!("Expected: {}", join_expected(expected));

    let diag = match error {
        InvalidToken { location } => Diagnostic::error()
            .with_message("Invalid token")
            .with_labels(vec![Label::primary(file_id, *location..*location)]),
        UnrecognizedEOF { location, expected } => Diagnostic::error()
            .with_message("Unexpected EOF")
            .with_labels(vec![
                Label::primary(file_id, *location..*location).with_message(fmt_expected(expected))
            ]),
        UnrecognizedToken {
            token: (start, _tok, end),
            expected,
        } => Diagnostic::error()
            .with_message("Unrecognized token")
            .with_labels(vec![
                Label::primary(file_id, *start..*end).with_message(fmt_expected(expected))
            ]),
        ExtraToken {
            token: (start, _tok, end),
        } => Diagnostic::error()
            .with_message("Extra token")
            .with_labels(vec![Label::primary(file_id, *start..*end)])
            .with_message("Extra token"),
        User { error } => Diagnostic::error()
            .with_message("Invalid token")
            .with_labels(vec![Label::primary(file_id, error.0.clone())])
            .with_message("Invalid token"),
    };

    (files, diag)
}
