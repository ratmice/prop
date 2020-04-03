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

    let join_expected = |expected: &Vec<String>| -> String {
        if let Some((caboose, rest)) = expected.split_last() {
            if rest.is_empty() {
                format!("Expected: {}", &caboose)
            } else {
                format!("Expected: {} or {}", rest.join(", "), &caboose)
            }
        } else {
            // ??
            "Didn't know what to expect?".to_string()
        }
    };

    let diag = match error {
        InvalidToken { location } => Diagnostic::error()
            .with_message("Invalid token")
            .with_labels(vec![Label::primary(file_id, *location..*location)]),
        UnrecognizedEOF { location, expected } => Diagnostic::error()
            .with_message("Unexpected EOF")
            .with_labels(vec![
                Label::primary(file_id, *location..*location).with_message(join_expected(expected))
            ]),
        UnrecognizedToken {
            token: (start, _tok, end),
            expected,
        } => Diagnostic::error()
            .with_message("Unrecognized token")
            .with_labels(vec![
                Label::primary(file_id, *start..*end).with_message(join_expected(expected))
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
