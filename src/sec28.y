%start File 
%nonassoc "<=>"
%right "=>" "|" "&"
%nonassoc "fst" "snd" "inl" "inr" "abort"
%nonassoc "~"
// %avoid_insert "INT"
%%

File
: D
| File ";" D
;

D
: "proof" "x" ":" A "=" "begin" P "end"
| "annotated" "proof" "x" ":" A "=" "begin" P "end"
| "term" "x" ":" A "=" M
|
;

P
: E
| P ";" E 
;

H: "x" ":" A;

// Proof entry. 
E
: M ":" A 
| "[" H ";" P "]"
|
;

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

M
: M_term
| M M_term
;

M_term
: "x"
| "(" M_parenthesized ")"
| "fst" M_term
| "snd" M_term
| "inl" M_term
| "inr" M_term
| "case" M_term "of" "inl" "x" "=>" M_term "|" "inr" "x" "=>" M_term "end"
| "fn" "x" "=>" M_term
| "abort" M_term
;

M_parenthesized
: M "," M
| M
| M ":" A
|
;

%%
// Any functions here are in scope for all the grammar actions above.
