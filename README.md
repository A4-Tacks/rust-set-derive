# Using macro to implement the comprehensions of imitating python language in rust
- You can initialize common collection types with relatively little code
- `Vec [v]`, `HashMap {k=>v}` and `HashSet !{k}` have been implemented

## Vec Or HashSet Syntax
- Ordinary element: `[foo, 1, 2i32]`
- for each: `[*(i; for i in 0..10)]`
- for each and filter: `[*(i; for i in 0..10; if i != 5)]`
- for each and filter and sub for each:
  `[*(i + j; for i in 0..10; if i != 5; for j in 1..i)]`


## HashMap Syntax
- Ordinary element: `{k => v, [k1]: v1}`
- String ordinary key: `{%[key]: "jack"}` (`key: String`)
- Ident key: `{name: "jack"}` (`name: &'static str`)
- String ident key: `{%name: "jack"}` (`name: String`)
- Variable key: `{*key: "jack"}` (`let key = "name";`)
- Literal key: `{"name": "jack"}`
- Literal key tostring: `{%"name": "jack"}` (`name: String`)
- for each: Just like in Vec, but slightly different.
  e.g `{*(i => i+1; for i in 0..5)}`


## Other
- for each reserve: `[*[10](i; for i in 0..10)]`
- generic params: `<i64>[1, 2, 3]`
- init with capacity: `(20)[1, 2, 3]` | `<i64>(20)[1, 2, 3]`
- inspect block: `[*(i; for i in 0..10; {dbg!(i);} if i & 1 == 0)]`


crate: <https://crates.io/crates/set_derive>


# Examples
```rust
# use set_derive::set_derive;
# use std::collections::HashMap;
let arr1 = vec![
    0, 1, 8, 9, 10, 15, 16, 17, 18, 19,
];
let arr2 = set_derive!([
    0, 1,
    *(i; for i in 8..=10),
    *(i; for i in 15..20),
]);
// python: [0, 1, *range(8, 10+1), *range(15, 20)]
assert_eq!(arr1, arr2);

let arr = set_derive!([*(
    i+j;
    for i in 0..6;
    if i%2!=0;
    for j in 0..i;
)]);
// python: [i+j for i in range(6) if i%2 for j in range(0, i)]
assert_eq!(arr, vec![1, 3, 4, 5, 5, 6, 7, 8, 9]);

let age_key = "age";
let jack = set_derive!({
    name: "jack",
    *age_key: "20",
});
assert_eq!(jack, HashMap::from([("name", "jack"), ("age", "20")]));
```
