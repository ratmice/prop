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
    let diag = match error {
        InvalidToken { location } => Diagnostic::error()
            .with_message("Invalid Token")
            .with_labels(vec![Label::primary(file_id, *location..*location)]),
        UnrecognizedEOF { location, expected } => Diagnostic::error()
            .with_message("Unrecognized EOF")
            .with_labels(vec![Label::primary(file_id, *location..*location)])
            .with_message(expected.join(" ")),
        UnrecognizedToken {
            token: (start, _tok, end),
            expected,
        } => Diagnostic::error()
            .with_message("Unrecognized Token")
            .with_labels(vec![Label::primary(file_id, *start..*end)])
            .with_message(expected.join(" ")),
        ExtraToken {
            token: (start, _tok, end),
        } => Diagnostic::error()
            .with_message("Extra token")
            .with_labels(vec![Label::primary(file_id, *start..*end)])
            .with_message("Extra token"),
        User { error } => Diagnostic::error()
            .with_message("Invalid Token")
            .with_labels(vec![Label::primary(file_id, error.0.clone())])
            .with_message("Invalid token"),
    };

    (files, diag)
}
