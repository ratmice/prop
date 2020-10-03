%start File 
%nonassoc "<=>"
%right "=>" "|" "&"
%right "*" "+" "->"
%nonassoc "list"
%right "~"
%%

File
: D
| File ";" D
;

Ty
: "1"
| "nat"
| "bool"
| Ty "list"
| Ty "*" Ty
| Ty "+" Ty
| Ty "->" Ty
| A_ty
// Covered by A_ty
// | "0"
;

D
: "proof" "x" ":" A ":=" "begin" P "end"
| "proof" "Q" ":" Ty ":=" "begin" P "end"
| "classical" "proof" "x" ":" A ":=" "begin" P "end"
| "classical" "proof" "Q" ":" Ty ":=" "begin" P "end"
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

A_tail
: "T"
| "F"
| "Q"
| "Q" M //  R M1...Mn/R(M1..Mn) (I think)
| M "=" M
| M "<" M
| "(" A ")"
;

A_op
: A_op "<=>" A_op
| A_op "=>" A_op
| A_op "|" A_op
| A_op "&" A_op
| "~" A_op
| A_op "|" A_quant A_op
| A_op "&" A_quant A_op
| A_op "<=>" A_quant A_op
| A_op "=>" A_quant A_op
| "~" A_quant A_op
| A_tail
;

A_quant
: "!" "x" ":" Ty "."
| "?" "x" ":" Ty "."
| A_quant "!" "x" ":" Ty "."
| A_quant "?" "x" ":" Ty "."
;

A
: A_quant A_op
| A_op
;

A_tail_ty
: "T"
| "F"
| "x"
| "Q"
| "Q" M //  R M1...Mn/R(M1..Mn) (I think)
| M_or_x "=" M_or_x
| M_or_x "<" M_or_x 
| "(" Ty ")"
;

M_or_x
: x
| M_no_x
;

A_op_ty
: A_op_ty "<=>" A_op_ty
| A_op_ty "=>" A_op_ty
| A_op_ty "|" A_op_ty
| A_op_ty "&" A_op_ty
| "~" A_op_ty
| A_op_ty "<=>" A_quant_ty A_op_ty
| A_op_ty "=>" A_quant_ty A_op_ty
| A_op_ty "|" A_quant_ty A_op_ty
| A_op_ty "&" A_quant_ty A_op_ty
| "~" A_quant A_op_ty
| A_tail_ty
;

A_quant_ty
: "!" "x" ":" Ty "."
| "?" "x" ":" Ty "."
| A_quant_ty "!" "x" ":" Ty "."
| A_quant_ty "?" "x" ":" Ty "."
;

A_ty
: A_quant_ty A_op_ty
| A_op_ty
;

M
: M_term
| M M_term
| M "::" M_term
;

z_paren
: "0"
| "(" z_paren ")"
;

s_paren
: "s" "x"
| "s" "(" "x" ")"
| "(" s_paren ")"
;

nil_paren
: "nil"
| "(" nil_paren ")"
;

x_paren
: "x"
| "(" x_paren ")"
;

M_term
: "x"
| "0"
| "s" M_term
| "rec" M "of" "x" z_paren "=>" M "|" "x" s_paren "=>" M "end"
| "rec" M "of" "x" nil_paren "=>" M "|" "x" "(" "x" "::" "x" ")" "=>" M "end"
| "true"
| "false"
| "if" M "then" M "else" "0"
| "nil"
| "(" M_parenthesized ")"
| "fst" M_term
| "snd" M_term
| "inl" M_term
| "inr" M_term
| "let" "(" "x" "," "x" ")" "=" M "in" M_term
| "case" M "of" "inl" x_paren "=>" M "|" "inr" x_paren "=>" M "end"
| "fn" "x" "=>" M_term
| "abort" M_term
| "eq0"
| "eqS" M_term
| "eqE0S" M_term
| "eqES0" M_term
| "eqESS" M_term
| "less0" M_term
| "lessS" M_term
| "lessE0" M_term
| "lessES" M_term
| "eqN"
| "eqC" M_term
| "eqENC" M_term
| "eqECN" M_term
| "eqECC" M_term
;

M_parenthesized
: M "," M
| M
| M ":" A
|
;

M_no_x
: M_term_no_x
| M_no_x M_term_no_x
| M_no_x "::" M_term_no_x
;

M_term_no_x
: "0"
| "s" M_term
| "rec" M "of" "x" z_paren "=>" M "|" "x" s_paren "=>" M "end"
| "rec" M "of" "x" nil_paren "=>" M "|" "x" "(" "x" "::" "x" ")" "=>" M "end"
| "true"
| "false"
| "if" M "then" M "else" "0"
| "nil"
| "(" M_no_x_parenthesized ")"
| "fst" M_term
| "snd" M_term
| "inl" M_term
| "inr" M_term
| "let" "(" "x" "," "x" ")" "=" M "in" M_term
| "case" M "of" "inl" x_paren "=>" M "|" "inr" x_paren "=>" M "end"
| "fn" "x" "=>" M_term
| "abort" M_term
| "eq0"
| "eqS" M_term
| "eqE0S" M_term
| "eqES0" M_term
| "eqESS" M_term
| "less0" M_term
| "lessS" M_term
| "lessE0" M_term
| "lessES" M_term
| "eqN"
| "eqC" M_term
| "eqENC" M_term
| "eqECN" M_term
| "eqECC" M_term
;

M_no_x_parenthesized
: M_no_x "," M_no_x
| M_no_x
| M_no_x ":" A
|
;

%%
// Any functions here are in scope for all the grammar actions above.
