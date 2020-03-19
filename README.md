# Prop (name should probably change)

Is intended to be a simplish but relatively complete example for intuitionistic propositional logic.
It is a work in progress, and not as simple as it should be.

## Under what does it purport to show? 

* integrating the [lalrpop](http://lalrpop.github.io/lalrpop/) parser generator with an external lexer generator.
* Fancy error messages with [codespan-reporting](https://github.com/brendanzab/codespan).
* Lexing unicode with [logos](https://github.com/maciejhirsz/logos)


## Syntax 

It tries to use a bidirectional Unicode ↔ ASCII syntax,
This should eventually include a code formatter for converting between character sets.
This should be bijective in that converting from ASCII → Unicode → ASCII should produce a file exactly equivalent to the first input.
This should eventually include a user-defined unicode ↔ ASCII map for user defined symbols.

Variables which start with the '\' character, are intended to have both a unicode and an ascii sequence (starting with '\').
This should be controllable via something like a `JSON` or [Ron](https://crates.io/crates/ron) file the code formatter will read.
So as not to be limited to the built-in unicode symbols.

# Future

We strive to keep the codebase simple enough to be used as a parsing example, but if it does not detract from that purpose.
It may eventually grow to include:

* Actual checking of proofs, As a proof checker it should take inspiration primarily from [tutch](http://www2.tcs.ifi.lmu.de/~abel/tutch/).
* A mechanism for extracting markdown documentation comments, with automagic pdf generation via [lumpy](https://github.com/ratmice/lumpy-leandoc).
* Mixfix is a possibility

if these features do turn out to detract from that goal, this should be implemented in a companion program sharing the syntax.

