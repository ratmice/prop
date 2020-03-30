use logos::Logos;

// Notably absent from the above, present in the below are
// Whitespace, EOF, LexError
#[derive(Logos, Debug)]
pub enum Token {
    #[end]
    EOF,

    #[token = "."]
    Dot,

    #[token = ":="]
    #[token = "≔"]
    Def,

    // FIXME pick only one of these.
    // perhaps we can do some proc-macro stuff to choose from a map file.
    // having multiple tokens here impedes it from being a single bijection
    // so we either need to pick one, or build a way to deal with multiple bijections.
    //
    // I find -> harder to type even though its standard, readable, and shorter.
    // So choosing \to for now, but that is not a good argument for it
    // being the default.
    //
    // #[token = "->"]
    #[token = r"\to"]
    #[token = "→"]
    Arrow,

    #[token = r"\iff"]
    #[token = "↔"]
    Iff,

    #[token = r"\neg"]
    #[token = "¬"]
    Neg,

    // note not the greek lambda character but the coptic character.
    #[token = r"\lam"]
    #[token = "ⲗ"]
    Abs,

    #[token = r"\and"]
    #[token = "∧"] // Not ^
    Conj,

    #[token = r"\or"]
    #[token = "∨"] // Not v compare: ∨v
    Disj,

    #[token = r"\top"]
    #[token = "⊤"]
    Top,

    #[token = r"\bot"]
    #[token = "⊥"]
    Bot,

    #[token = "("]
    LParen,
    #[token = ")"]
    RParen,

    // Since this uses Coptic letters for keywords all greek letters can be used as variable names.
    // Variables can start with a slash character, a greek/math alphanumeric symbol,
    // and ascii letters numbers, and subscripts (TODO superscripts)
    //
    // The above are commented out because of their affect on compile times...
    // In some cases they cause the proc-macro to overflow the stack
    // Essentially you should think of this like:
    //
    // [\\a-zA-Z][a-zA-Z0-9]
    //
    // I.e. the normal thing + a \.
    // then it just adds a bunch of math letters to the first [...]
    // and subscripts to the second.
    //
    // \x{2100}-\x{214f} Letter-like symbols.
    // A subset of the Mathematical Alphanumeric symbols -- Mathematical Script, Fractur, Double-Struck
    // https://unicode.org/charts/PDF/U1D400.pdf
    // \x{1d49c}-\x{1d59f}
    //
    // Subscripts:
    // \x{2080}-\x{2089}
    // \x{2090}-\x{209c}
    // \x{1d62}-\x{1d6a}
    //
    // FancyNameAscii ↔ FancyNameUnicode
    #[regex = r"[\\][a-zA-Z][_a-zA-Z0-9]*"]
    FancyNameAscii,
    #[regex = r"[a-zA-Z\p{Greek}\x{1d49c}-\x{1d59f}\x{2100}-\x{214f}][_a-zA-Z0-9\x{207f}-\x{2089}\x{2090}-\x{209c}\x{1d62}-\x{1d6a}]*"]
    Name,

    #[token = ":"]
    Colon,

    #[token = ";"]
    Semi,

    #[regex = r"#.*\n"]
    Comment,

    #[regex = r"\p{Whitespace}+"]
    Whitespace,

    #[error]
    LexError,
}

