use crate::error::*;
use crate::lex;
use crate::{parser, test_util};
use logos::Logos;

use unindent::unindent;

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
fn bad_unicode() -> Result<(), MainError> {
    let invalid_source = [
        r"ₐₑₒₓₔₕₖₗₘₙₚₛₜ₀₁₂₃₄₅₆₇₈₉ ≔ ⊤", // Subscript cannot be initial character
        r"\α ≔ ⊤", // Unicode cannot start with slash.
        r"SomeInvalidSyntaxEndingInUnicodeₐ fooₐ ≔ ⊤",
        r"ℤSomeInvalidSyntaxₐ ℤfooₐ ≔ ⊤",
    ];

    Ok(test_util::expect_fail(test_util::do_test(&invalid_source))?)
}

#[test]
fn thus() -> Result<(), MainError> {
    let source = [
        r"",
        &unindent(
            r#"ab_or_cd : (A ∧ B) ∨ (C ∧ D) → B ∨ D
           ≔ ‣ (A ∧ B) ∨ (C ∧ D)          ;; +⚡1
               ‣ A ∧ B                    ;; +⚡2
                 ∴ B                      ;;      ∴ ∧E R
                 ∴ B ∨ D.                 ;;      ∴ ∨I L
               ‣ C ∧ D                    ;; +⚡2
                 ∴ D                      ;;      ∴ ∧E R
                 ∴ B ∨ D.                 ;;      ∴ ∨I R
             ∴ B ∨ D.                     ;; -⚡2 ∴ ∨E
             ∴ (A ∧ B) ∨ (C ∧ D) → B ∨ D. ;; -⚡1 ∴ →I
           ;
        "#,
        ),
        &unindent(
            r#"ab_or_cd : (A ∧ B) ∨ (C ∧ D) ↔ (B ∧ A) ∨ (D ∧ C)
           ≔ ‣‣(A ∧ B) ∨ (C ∧ D)                        ;; +⚡1, +⚡2
               ‣ A ∧ B                                  ;; +⚡3
                 ∴ B                                    ;; ∴ ∧E R
                 ∴ A                                    ;; ∴ ∧E L
                 ∴ B ∧ A                                ;; ∴ ∧I
                 ∴ (B ∧ A) ∨ (D ∧ C).                   ;; ∴ ∨I L
               ‣ C ∧ D                                  ;; +⚡3
                 ∴ D                                    ;; ∴ ∧E R
                 ∴ C                                    ;; ∴ ∧E L
                 ∴ D ∧ C                                ;; ∴ ∧I
                 ∴ (B ∧ A) ∨ (D ∧ C).                   ;; ∴ ∨I R
               ∴ (B ∧ A) ∨ (D ∧ C).                     ;; -3 ∴ ∨E
              ∴ (A ∧ B) ∨ (C ∧ D) → (B ∧ A) ∨ (D ∧ C).  ;; -2 ∴ →I

             ‣‣ (B ∧ A) ∨ (D ∧ C)                       ;; +⚡1, +⚡2
               ‣ B ∧ A                                  ;; +⚡3
                 ∴ B ∧ A ∴ A                            ;; ∴ ∧E R
                 ∴ B ∧ A ∴ B                            ;; ??, ∴ ∧E L
                 ∴ A ∧ B                                ;; ∴ ∧I
                 ∴ (A ∧ B) ∨ (C ∧ D).                   ;; ∴ ∨I L
               ‣ D ∧ C                                  ;; +⚡3
                 ∴ C                                    ;; ∴ ∧E R
                 ∴ D                                    ;; ??,  ∴ ∧E L
                 ∴ C ∧ D                                ;; ∴ ∧I
                 ∴ (A ∧ B) ∨ (C ∧ D).                   ;; ∴ ∨I R
               ∴ (A ∧ B) ∨ (C ∧ D).                     ;; -⚡3 ∴ ∨E
              ∴ (B ∧ A) ∨ (D ∧ C) → (A ∧ B) ∨ (C ∧ D).  ;; -⚡2 ∴ →I

             ∴ (A ∧ B) ∨ (C ∧ D) ↔ (B ∧ A) ∨ (D ∧ C).   ;; -⚡1 ∴ ↔I
           ;
        "#,
        ),
    ];
    Ok(test_util::expect_success(test_util::do_test(&source))?)
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
        let lex = lex::Token::lexer(&s).spanned();
        let parse_result = parser::propParser::new().parse(lex.map(|(t, r)| {
            if t == lex::Token::LexError {
                Err(r.start)
            } else {
                Ok((r.start, t, r.end))
            }
        }));

        match parse_result {
            Ok(_) => {
                // bad
                println!("parsed but shouldn't: {}", s);
                num_fail += 1;
            }
            Err(e) => {
                // good
                println!("expected error: {:?}", e);
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
