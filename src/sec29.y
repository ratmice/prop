%start File 
%nonassoc "<=>"
%right "=>" "|" "&"
%right "*" "+" "->"
%nonassoc "list"
%nonassoc "fst" "snd" "inl" "inr" "abort"
%nonassoc "~"
// %avoid_insert "INT"
%%

File
: D
| D ";" File
;

T
: "1"
| "0"
| A_t
| "nat"
| "bool"
| T "list"
| T "*" T
| T "+" T
| T "->" T
;


D
: "proof" "x" ":" A "=" "begin" P "end"
| "annotated" "proof" "x" ":" A "=" "begin" P "end"
| "term" "x" ":" A "=" M
| "val" "x" ":" T "=" M
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

A_np_terminal
: "T"
| "F"
| "Q"
;

A_terminal_t
: A_np_terminal
| "(" T ")"
;

A_terminal
: A_np_terminal
| "(" A ")"
;

A_t
: A_t "<=>" A_terminal_t
| A_t "=>" A_terminal_t
| A_t "|" A_terminal_t
| A_t "&" A_terminal_t
| "~" A_t
| A_terminal_t
;

A
: A "<=>" A_terminal
| A "=>" A_terminal
| A "|" A_terminal
| A "&" A_terminal
| "~" A
| A_terminal
;

M
: M_term
| M M_term
| M "::" M_term
;

M_term
: "x"
| "0"
| "s" M_term
| "rec" M_term "of" "x" "0" "=>" M_term "|" "x" "(" "s" "x" ")" "=>" M_term "end"
| "rec" M_term "of" "x" "nil" "=>" M_term "|" "x" "(" "x" "::" "x" ")" "=>" M_term "end"
| "true"
| "false"
| "if" M_term "then" M_term "else" "0"
| "nil"
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
