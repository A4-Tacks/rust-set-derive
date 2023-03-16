# Using macro to implement the comprehensions of imitating python language in rust
- You can initialize common collection types with relatively little code
- `Vec [v]`, `HashMap {k=>v}` and `HashMap {k}` have been implemented
- VecSyntax: `(<types>)?(\(capacity\))?([result; for\(...\)((for|if)\(...\))*])`

# Examples
```
use set_derive::*;
# use std::{iter::zip,collections::{HashMap,HashSet}};

assert_eq!(set_derive!((8)[1; for(_ in 0..1)]).capacity(), 8);

let a = set_derive!(<i64>[i; for(i in 0..5) if(i & 1 == 0)]);
assert_eq!(a, vec![0, 2, 4]);

let a = set_derive!(<i16>(30)[j; for(i in 0..5) if(i & 1 == 0)
                            for (j in 0..=i)]);
assert_eq!(a.capacity(), 30);
assert_eq!(a, vec![0,0,1,2,0,1,2,3,4]);

let a = set_derive!({j; for(i in 0..5) if(i & 1 == 0)
                            for (j in 0..=i)});
assert_eq!(a, HashSet::from_iter(0..5));

let a = set_derive!({j => j + 1; for(i in 0..5)
                        if(i & 1 == 0) for(j in 0..=i)});
assert_eq!(a, HashMap::from_iter(zip(0..5, 1..6)));
```
