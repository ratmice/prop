use unindent::unindent;
use crate::error::*;
use crate::{parser, lex, test_util};
#[test]
fn pretty_errors() -> Result<(), MainError> {
    let source = [
        "trivial := .",
        &unindent(
            r#"
            x := x;
            y := y;
            lexical_error := .;
        "#,
        ),
    ];

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
        &unindent(
            r##"
            # This is a comment
            A ≔ ⊤
        "##,
        ),
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
        &unindent(
            r#"
            A := \top;
            B := \top
        "#,
        ),
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
