initSidebarItems({"enum":[["Column","A type representing one of the polynomials involved in the PLONK IOP."],["ConstantExpr","An arithmetic expression over"],["Expr","An multi-variate polynomial over the base ring `C` with variables"],["Op2","A binary operation"],["PolishToken","For efficiency of evaluation, we compile expressions to reverse Polish notation expressions, which are vectors of the below tokens."]],"fn":[["coeff",""],["index","Handy function to quickly create an expression for a gate."],["l0_1","Computes `prod_{j != 1} (1 - omega^j)`"],["pows","Compute the powers of `x`, `x^0, ..., x^{n - 1}`"],["witness","Handy function to quickly create an expression for a witness."],["witness_curr","Same as [witness] but for the next row."],["witness_next","Same as [witness] but for the next row."]],"mod":[["constraints","A number of useful constraints"],["prologue","You can import this module like `use kimchi::circuits::expr::prologue::*` to obtain a number of handy aliases and helpers"]],"struct":[["Cache","A cache"],["CacheId","A key for a cached value"],["Constants","The collection of constants required to evaluate an `Expr`."],["Environment","The collection of polynomials (all in evaluation form) and constants required to evaluate an expression as a polynomial."],["Linearization","A “linearization”, which is linear combination with `E` coefficients of columns."],["LookupEnvironment","The polynomials specific to the lookup argument."],["Variable","A type representing a variable which can appear in a constraint. It specifies a column and a relative position (Curr or Next)"]],"type":[["E","An alias for the intended usage of the expression type in constructing constraints."]]});