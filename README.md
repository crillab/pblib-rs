# pblib-rs

Rust safe bindings for pblib.

The source code of pblib is taken from the fork maintained by Radomír Černoch [on Github](https://github.com/master-keying/pblib).
This crate currently provides bindings to a partial set of the initial library functions.
See the original documentation (that can be retrieved from the fork's README page) if you need more information on the encodings.

The entry point of the API is the `PB2CNF` structure.
You should begin by taking a look at its documentation is you want to use pblib-rs.

## TL;DR

Cardinality constraints:

```rust
use pblib_rs::PB2CNF;

// we encode x1 + x2 + x3 >= 2
let literals = vec![1, 2, 3];
let pb2cnf = PB2CNF::new();
// the threshold is 2 and the first variable not in use is 4
let encoding = pb2cnf.encode_at_least_k(literals, 2, 4);
println!("the encoding uses {} variables", encoding.next_free_var_id() - 4);
println!("the encoding uses {} clauses", encoding.clauses().len());
encoding.clauses().iter().enumerate().for_each(|(i,c)| println!("clause {i} is {:?}", c));
```

Pseudo-Boolean constraints:

```rust
use pblib_rs::PB2CNF;

// we encode 8*x1 + 4*x2 + 2*x3 + 1*x4 >= 6
let weights = vec![8, 4, 2, 1];
let literals = vec![1, 2, 3, 4];
let pb2cnf = PB2CNF::new();
// the threshold is 6 and the first variable not in use is 5
let encoding = pb2cnf.encode_geq(weights.clone(), literals, 6, 5);
println!("the encoding uses {} variables", encoding.next_free_var_id() - 4);
println!("the encoding uses {} clauses", encoding.clauses().len());
encoding.clauses().iter().enumerate().for_each(|(i,c)| println!("clause {i} is {:?}", c));
```

## License

pblib-rs is developed at CRIL (Univ. Artois & CNRS).
It is made available under the terms of the GNU Lesser GPLv3 license.
