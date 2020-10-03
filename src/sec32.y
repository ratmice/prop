%start File 
%nonassoc "<=>"
%right "=>" "|" "&"
%right "*" "+" "->"
%nonassoc "list"
%nonassoc "fst" "snd" "inl" "inr" "abort"
%nonassoc "~"
%nonassoc "?" "!" ":"
// %avoid_insert "INT"
%%

File
: D
| File ";" D
;

Ty
: "1"
| "0"
| "nat"
| "bool"
| Ty "list"
| Ty "*" Ty
| Ty "+" Ty
| Ty "->" Ty
| A_t
;

D
: "proof" "x" ":" A ":=" "begin" P "end"
| "proof" "Q" ":" Ty ":=" "begin" P "end"
| "annotated" "proof" "x" ":" A ":=" "begin" P "end"
| "annotated" "proof" "Q" ":" A ":=" "begin" P "end"
| "term" "x" ":" A ":=" M
| "val" "x" ":" Ty ":=" M
|
;

P
: E
| P ";" E 
;

H
: "x" ":" Ty
| A
;

Hs
: H
| Hs "," H
;

// Proof entry. 
E
: A
| M ":" Ty 
| "[" Hs ";" P "]"
|
;

A_terminal_nr
: "T"
| "F"
| "Q" M_or_empty //  R M1...Mn/R(M1..Mn) (I think)
| "~" A_op
| M "=" M
| M "<" M
| A_quant
;

A_terminal
: A_terminal_nr
| "(" A ")"
;

A_op
: A_op "<=>" A_op
| A_op "=>" A_op
| A_op "|" A_op
| A_op "&" A_op
| A_terminal
;

A_op_foo
: "T"
| "F"
| "Q" M_or_empty //  R M1...Mn/R(M1..Mn) (I think)
| "(" A_quant ")"
;

A_op_bar
: "~" A_op_foo
| A_op_bar "<=>" A_op_bar
| A_op_bar "=>" A_op_bar
| A_op_bar "|" A_op_bar
| A_op_bar "&" A_op_bar
| A_op_foo
;

A_quant
: "!" "x" ":" Ty "." A_terminal_nr
| "!" "x" ":" Ty "." "(" A_terminal_nr ")"
| "?" "x" ":" Ty "." A_terminal_nr
| "?" "x" ":" Ty "." "(" A_terminal_nr ")"
;
/*
A_quant
: "!" "x" ":" Ty "." A_quant
| "?" "x" ":" Ty "." A_quant
| A_op_bar
;
*/

/*
A_quant
: "!" "x" ":" Ty "." A_quant
| "!" "x" ":" Ty "." A_op_foo
| "!" "x" ":" Ty "." "(" A_quant ")"
| "!" "x" ":" Ty "." "(" A_op_foo ")"
| "?" "x" ":" Ty "." A_quant
| "?" "x" ":" Ty "." "(" A_quant ")"
| "?" "x" ":" Ty "." A_op_foo
| "?" "x" ":" Ty "." "(" A_op_foo ")"
;
*/

A: A_op;

A_terminal_t_nr
: "T"
| "F"
| "x"
| "Q" M_or_empty //  R M1...Mn/R(M1..Mn) (I think)
| "~" A_op_t
| A_quant_t
;

A_terminal_t
: A_terminal_t_nr
| "(" Ty ")"
;

M_or_empty
:
| M
;

A_op_t_nr
: A_op_t "<=>" A_op_t
| A_op_t "=>" A_op_t
| A_op_t "|" A_op_t
| A_op_t "&" A_op_t
;

A_op_t
: A_op_t_nr
| A_terminal_t
;

A_op_t_nr_terminal_nr
: A_op_t_nr
| A_terminal_t_nr;

A_quant_t
: "!" "x" ":" Ty "." A_terminal_t_nr
| "!" "x" ":" Ty "." "(" A_terminal_t_nr ")"
| "?" "x" ":" Ty "." A_terminal_t_nr
| "?" "x" ":" Ty "." "(" A_terminal_t_nr ")"
;

A_t: A_op_t;
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
