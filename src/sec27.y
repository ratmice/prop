%start File 
// %precedence "<=>" "=>" "|" "&" "~"
%nonassoc "<=>"
%right "=>" "|" "&"
%nonassoc "~"
// %avoid_insert "INT"
%%

A
: "T"
| "F"
| "Q"
| A "<=>" A
| A "=>" A
| A "|" A
| A "&" A
| "~" A
| "(" A ")"
;

// Hypothesis
H
: A
;

// Proof entry. 
E
: A
| "[" H ";" P "]"
|
;

P
: E
| P ";" E
;

D
: "proof" "x" ":" A "=" "begin" P "end"
|
;

File
: D
| File ";" D
;

%%
// Any functions here are in scope for all the grammar actions above.
