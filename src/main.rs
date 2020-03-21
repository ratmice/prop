mod ast;
mod codespan;
mod error;
mod lex;
#[cfg(test)]
mod test_util;

use codespan_reporting::term::termcolor::StandardStream;
use codespan_reporting::term::{self, ColorArg};
use error::*;
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

mod rowan_parser {
    // Hack to avoid clippy lints in generated code.
    #![allow(clippy::all)]
    use lalrpop_util::lalrpop_mod;
    lalrpop_mod!(rowan_prop);
    pub use rowan_prop::*;
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

#[test]
fn pretty_errors() -> Result<(), MainError> {
    let source = ["trivial := ."];

    Ok(test_util::expect_fail(test_util::do_test(&source))?)
}

#[test]
fn stuff() -> Result<(), MainError> {
    let source = [
        "trivial ≔ ⊤",
        "id: A → A ≔ ⲗ a. a",
        "demorgan1 ≔ ¬(A ∨ B) ↔ (¬A) ∧ (¬B)",
        "demorgan2 ≔ ¬(A ∧ B) ↔ (¬A) ∨ (¬B)",
        "const: A → B → A ≔ ⲗa. ⲗb. a",
        "flip: (A → B → C) → B → A → C ≔ ⲗf. ⲗb. ⲗa. f a b",
        r##"
            # This is a comment
            A ≔ ⊤
        "##,
    ];

    Ok(test_util::expect_success(test_util::do_test(&source))?)
}

#[test]
fn unicode_vars() -> Result<(), MainError> {
    let source = [
        r"𝓞 ≔ ⊤",
        r"α ≔ ⊤",
        r"𝕎 ≔ x",
        r"ℕ ≔ x",
        r"ℤ ≔ x",
        r"LotsOfSubscriptsₐₑₒₓₔₕₖₗₘₙₚₛₜ₀₁₂₃₄₅₆₇₈₉ ≔ x",
        r"ℤ ≔ x",
        r"Foo: ⊤ ≔ x",
        r"λ ≔ A ∨ ¬A",
    ];

    Ok(test_util::expect_success(test_util::do_test(&source))?)
}

#[test]
#[cfg_attr(all(test, feature = "ignore_kfail_tests"), ignore)]
fn fixme() -> Result<(), MainError> {
    let source = [
        "", // placeholder,
        r#"id ≔ (ⲗx. x) (ⲗx. x)"#,
        "modus_ponens: (A → B) → A → B ≔ ⲗf. ⲗa. f a",
        "foo ≔ ¬((A ∨ B) ↔ (A ∧ B))",
        // surely there is more to be fixed.
    ];
    Ok(test_util::expect_success(test_util::do_test(&source))?)
}

#[test]
fn bad_unicode() -> () {
    let invalid_source = [
        r"ₐₑₒₓₔₕₖₗₘₙₚₛₜ₀₁₂₃₄₅₆₇₈₉ ≔ ⊤", // Subscript cannot be initial character
        r"\α ≔ ⊤", // Unicode cannot start with slash.
    ];

    for s in invalid_source.iter() {
        match parser::propParser::new().parse(lex::Tokens::from_string(s)) {
            Ok(_) => panic!(format!("accepted '{}'", s)),
            Err(e) => println!("got an expected error: {:?}", e),
        }
    }
}

#[test]
fn good_ascii() -> Result<(), MainError> {
    let source = [
        r"id := A \to A",
        r"A := \top",
        r#"
        A := \top;
        B := \top
        "#,
        r"\A := ⊤",
    ];
    Ok(test_util::expect_success(test_util::do_test(&source))?)
}

#[test]
fn bad_ascii() -> Result<(), &'static str> {
    let invalid_source = [
        r"\\A ≔ ⊤",     // Slash can only be the initial character
        r"\to ≔ ⊤",     // \to is reserved for lambda
        r"x ≔ y ≔ ⊤", // y := t is not an expression.
    ];

    let mut num_fail = 0;
    for s in invalid_source.iter() {
        let lexer = lex::Tokens::from_string(&s);
        match parser::propParser::new().parse(lexer) {
            Ok(_) => {
                // bad
                println!("parsed but shouldn't: {}", s);
                num_fail += 1;
            }
            Err(e) => {
                // good
                println!("expected error: {}", e);
                ()
            }
        }
    }
    if num_fail == 0 {
        Ok(())
    } else {
        Err("received valid parse from supposedly invalid source")
    }
}

// type SyntaxNode = rowan::SyntaxNode<Lang>;
// type SyntaxToken = rowan::SyntaxToken<Lang>;
// type SyntaxElement = rowan::SyntaxElement<Lang>;

#[test]
fn rowan_lex() -> Result<(), error::MainError> {
    let s = "X := X";
    let lexer = lex::TokensRowan::from_string(&s);
    let mut builder = rowan::GreenNodeBuilder::new();

    builder.start_node(lex::LexToken::Root.into());
    let parse_result = rowan_parser::propParser::new().parse(&mut builder, tokens)?;
/*    for thing in lexer {
        let checkpoint = self.builder.checkpoint();
        println!("{:?}", thing);
    }
*/
    builder.finish_node();
    Ok(())
}

fn from_rowan<'a>(s: &'a str) -> Result<(), MainError> {
    let tokens = lex::TokensRowan::from_string(&s);
    let mut builder = rowan::GreenNodeBuilder::new();
    let parse_result = rowan_parser::propParser::new().parse(&mut builder, tokens);
    match parse_result {
        Err(e) => {
            println!("{:?}", e);
            Err(MainError::SomethingWentAwryAndStuffWasPrinted)
        }
        _ => Ok(())
    }
}

fn main() -> Result<(), MainError> {
    let mut buf = std::io::BufReader::new(std::io::stdin());
    let mut s = Box::new(String::new());

    // Not really how i'd like this to be.
    buf.read_to_string(&mut s)?;
    let lexer = lex::Tokens::from_string(&s);
    from_rowan(&s)?;
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
                // dump_binding(bind)
            }
        }
    }
    Ok(())
}
